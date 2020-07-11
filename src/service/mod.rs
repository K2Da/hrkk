use crate::error::Error::*;
use crate::error::Result;
use crate::opts::{Opts, SubCommand};
pub(crate) use serde::Serialize;
use serde_json::{Map, Value};
use yaml_rust::Yaml;

pub(crate) mod athena;
pub(crate) mod cloudwatch;
pub(crate) mod ec2;
pub(crate) mod logs;
pub(crate) mod prelude;
pub(crate) mod rds;
pub(crate) mod ssm;

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
        Box::new(athena::query_execution::new()),
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
    pub(crate) api_type: ApiType,
    pub(crate) document_url: &'static str,
}

#[derive(Serialize, Clone)]
pub(crate) enum ApiType {
    Xml(XmlApi),
    Json(JsonApi),
}

#[derive(Serialize, Clone)]
pub(crate) struct XmlApi {
    pub(crate) service_name: &'static str,
    pub(crate) action: &'static str,
    pub(crate) version: &'static str,
    pub(crate) limit_name: &'static str,
    pub(crate) iteration_tag: Vec<&'static str>,
    pub(crate) max_limit: i64,
}

#[derive(Serialize, Clone)]
pub(crate) struct JsonApi {
    pub(crate) service_name: &'static str,
    pub(crate) target: &'static str,
    pub(crate) json: serde_json::Value,
    pub(crate) limit_name: &'static str,
    pub(crate) token_name: &'static str,
    pub(crate) parameter_name: Option<&'static str>,
    pub(crate) max_limit: i64,
}

impl JsonApi {
    pub(crate) fn json_map(&self) -> Result<Map<String, Value>> {
        if let Value::Object(map) = &self.json {
            return Ok(map.clone());
        } else {
            Err(SettingError("request json is not a map.".to_string()))
        }
    }
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

    fn max_limit(&self) -> i64 {
        match self.info().api_type {
            ApiType::Xml(XmlApi { max_limit, .. }) => max_limit,
            ApiType::Json(JsonApi { max_limit, .. }) => max_limit,
        }
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
