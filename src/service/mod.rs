use crate::error::Error::*;
use crate::error::Result;
use crate::opts::{Opts, SubCommand};
use rusoto_core::signature::SignedRequest;
use rusoto_credential::ChainProvider;
pub(crate) use serde::Serialize;
use yaml_rust::Yaml;

pub(crate) mod cloudwatch;
pub(crate) mod ec2;
pub(crate) mod file;
pub(crate) mod json_helper;
mod json_to_yaml;
pub(crate) mod logs;
mod prelude;
pub(crate) mod rds;
pub(crate) mod ssm;
pub(crate) mod xml_helper;
mod xml_to_yaml;

pub(crate) fn all_resources() -> Vec<Box<dyn AwsResource>> {
    vec![
        Box::new(cloudwatch::alarm::new()),
        Box::new(cloudwatch::alarm_history::new()),
        Box::new(ec2::instance::new()),
        Box::new(logs::log_group::new()),
        Box::new(logs::log_stream::new()),
        Box::new(rds::db_instance::new()),
        Box::new(ssm::automation_execution::new()),
        Box::new(ssm::document::new()),
        Box::new(ssm::session::new()),
    ]
}

pub(crate) fn resource_by_name(name: &str) -> Box<dyn AwsResource> {
    for r in all_resources() {
        if r.name() == name {
            return r;
        }
    }
    panic!()
}

#[derive(Serialize)]
pub(crate) struct Info {
    key_attribute: &'static str,
    service_name: &'static str,
    resource_type_name: &'static str,
    api_type: ApiType,
    pub(crate) document_url: &'static str,
    pub(crate) max_limit: i64,
}

#[derive(Serialize, Clone)]
pub(crate) enum ApiType {
    Xml {
        service_name: &'static str,
        action: &'static str,
        version: &'static str,
        limit_name: &'static str,
        iteration_tag: Vec<&'static str>,
    },
    Json {
        service_name: &'static str,
        target: &'static str,
        json: serde_json::Value,
        limit_name: &'static str,
        token_name: &'static str,
        parameter_name: Option<&'static str>,
    },
}

impl Clone for Box<dyn AwsResource> {
    fn clone(&self) -> Self {
        resource_by_name(&self.name())
    }
}

pub(crate) trait AwsResource: Send + Sync {
    fn info(&self) -> &Info;

    fn matching_sub_command(&self) -> Option<SubCommand>;

    fn take_command(&self, sub_command: &SubCommand, _opts: &Opts) -> Result<ExecuteTarget> {
        match self.matching_sub_command() {
            Some(matching_command) => {
                if &matching_command == sub_command {
                    Ok(ExecuteTarget::ExecuteThis { parameter: None })
                } else {
                    Ok(ExecuteTarget::Null)
                }
            }
            None => panic!("should be overridden."),
        }
    }

    fn without_param(&self, _opts: &Opts) -> ExecuteTarget {
        ExecuteTarget::ExecuteThis { parameter: None }
    }

    fn make_vec(&self, yaml: &Yaml) -> (Vec<Yaml>, Option<String>);

    fn header(&self) -> Vec<&'static str>;

    fn line(&self, yaml: &Yaml) -> Vec<String>;

    fn detail(&self, yaml: &Yaml) -> crate::show::Section;

    fn name(&self) -> String {
        format!("{}_{}", self.service_name(), self.resource_type_name())
    }

    fn equal(&self, yaml: &Yaml, key: &str) -> bool {
        yaml[&self.key()[..]] == Yaml::String(key.to_string())
    }

    fn resource_name(&self, yaml: &Yaml) -> String {
        match &yaml[&self.key()[..]] {
            Yaml::String(resource_id) => resource_id.to_string(),
            _ => "no name".to_string(),
        }
    }

    fn service_name(&self) -> String {
        self.info().service_name.to_owned()
    }

    fn resource_type_name(&self) -> String {
        self.info().resource_type_name.to_owned()
    }

    fn resource_full_name(&self) -> String {
        format!("{}:{}", self.service_name(), self.resource_type_name())
    }

    fn command_name(&self) -> String {
        self.resource_type_name().replace("_", "-")
    }

    fn key(&self) -> String {
        self.info().key_attribute.to_owned()
    }

    fn response_type(&self) -> ApiType {
        self.info().api_type.clone()
    }
}

pub(crate) enum ExecuteTarget {
    ExecuteThis {
        parameter: Option<String>,
    },
    ParameterFromResource {
        param_resource: Box<dyn AwsResource>,
    },
    ParameterFromList {
        option_name: String,
        option_list: Vec<String>,
    },
    Null,
}

pub(crate) async fn execute_command(sub_command: &SubCommand, opts: Opts) -> Result<()> {
    use ExecuteTarget::*;
    for resource in all_resources() {
        match resource.take_command(sub_command, &opts.clone()) {
            Ok(ExecuteThis { parameter }) => {
                crate::ui::tui(opts.clone(), parameter, Some(resource)).await?;
            }
            Ok(ParameterFromResource { .. }) | Ok(ParameterFromList { .. }) => {
                crate::ui::tui(opts.clone(), None, Some(resource)).await?;
            }
            Ok(ExecuteTarget::Null) => (),
            Err(e) => return Err(e),
        }
    }
    Ok(())
}

pub(crate) fn tag_value<'a>(tags: &'a Yaml, name: &str) -> &'a Yaml {
    if let Yaml::Array(array) = tags {
        for tag in array {
            if Yaml::String(name.to_string()) == tag["key"] {
                return &tag["value"];
            }
        }
    }
    &Yaml::BadValue
}

pub(crate) fn next_token(yaml: &Yaml) -> Option<String> {
    match &yaml["next_token"] {
        Yaml::String(token) => Some(token.to_owned()),
        _ => None,
    }
}

pub(crate) async fn fetch(
    resource: &dyn AwsResource,
    parameter: &Option<String>,
    opts: &Opts,
    next_token: Option<String>,
) -> Result<(Vec<Yaml>, Option<String>)> {
    let request = request(resource, parameter, opts, next_token)?;

    let mut response =
        rusoto_core::Client::new_with(ChainProvider::default(), rusoto_core::HttpClient::new()?)
            .sign_and_dispatch(request)
            .await
            .map_err(|e| RusotoError(format!("{:?}", e)))?;

    let response = response
        .buffer()
        .await
        .map_err(|e| RusotoError(format!("{}", e)))?;

    if !response.status.is_success() {
        return Err(RusotoError(
            String::from_utf8(response.body.as_ref().to_vec()).unwrap_or("".to_string()),
        ));
    }

    if !response.body.is_empty() {
        if opts.debug {
            file::store_response(response.body.as_ref())?;
        }
        let yaml = match resource.response_type() {
            ApiType::Xml { iteration_tag, .. } => {
                xml_to_yaml::convert(response.body.as_ref(), &iteration_tag)?
            }
            ApiType::Json { .. } => json_to_yaml::convert(response.body.as_ref())?,
        };
        Ok(resource.make_vec(&yaml))
    } else {
        Err(RusotoError("response body is empty.".to_string()))
    }
}

fn request(
    resource: &dyn AwsResource,
    parameter: &Option<String>,
    opts: &Opts,
    next_token: Option<String>,
) -> Result<SignedRequest> {
    match &resource.info().api_type {
        ApiType::Xml { .. } => xml_helper::request(opts, next_token, resource),
        ApiType::Json { .. } => json_helper::request(opts, next_token, parameter, resource),
    }
}
