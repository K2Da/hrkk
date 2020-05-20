use crate::error::Error::*;
use crate::error::Result;
use crate::info::{BarInfo, CacheInfo, FetchInfo, ResourceInfo};
use crate::opts::{Opts, SubCommand};
use rusoto_core::signature::SignedRequest;
use rusoto_credential::ChainProvider;
pub use serde::Serialize;
use skim::prelude::*;
use yaml_rust::Yaml;

pub mod cloudwatch;
pub mod ec2;
pub mod file;
pub mod json_helper;
mod json_to_yaml;
pub mod logs;
mod prelude;
pub mod rds;
pub mod ssm;
pub mod xml_helper;
mod xml_to_yaml;

pub fn all_resources() -> Vec<Box<dyn AwsResource>> {
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

#[derive(Serialize)]
pub struct Info {
    key_attribute: &'static str,
    service_name: &'static str,
    resource_type_name: &'static str,
    api_type: ApiType,
    pub document_url: &'static str,
    pub max_limit: i64,
}

#[derive(Serialize, Clone)]
pub enum ApiType {
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

pub trait AwsResource: Send + Sync {
    fn info(&self) -> &Info;

    fn matching_sub_command(&self) -> Option<SubCommand>;

    fn take_command(&self, sub_command: &SubCommand, _opts: &Opts) -> Result<SkimTarget> {
        match self.matching_sub_command() {
            Some(matching_command) => {
                if &matching_command == sub_command {
                    Ok(SkimTarget::ExecuteThis { parameter: None })
                } else {
                    Ok(SkimTarget::None)
                }
            }
            None => panic!("should be overridden."),
        }
    }

    fn without_param(&self, _opts: &Opts) -> SkimTarget {
        SkimTarget::ExecuteThis { parameter: None }
    }

    fn make_vec(&self, yaml: &Yaml) -> (Vec<Yaml>, Option<String>);

    fn line(&self, yaml: &Yaml) -> Vec<String>;

    fn detail(&self, yaml: &Yaml) -> String;

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

pub enum SkimTarget {
    ExecuteThis {
        parameter: Option<String>,
    },
    ParameterFromResource {
        resource_name: String,
    },
    ParameterFromList {
        list: (String, Vec<(String, String)>),
    },
    None,
}

pub async fn execute_command(sub_command: &SubCommand, opts: &Opts) -> Result<()> {
    for resource in all_resources() {
        match resource.take_command(sub_command, opts) {
            Ok(SkimTarget::ExecuteThis { parameter }) => {
                execute_with_parameter(&*resource, &parameter, &opts).await?;
            }
            Ok(SkimTarget::ParameterFromResource { resource_name }) => {
                let param = parameter_from_resource(&resource_name, opts).await?;
                execute_with_parameter(&*resource, &param, opts).await?;
            }
            Ok(SkimTarget::ParameterFromList { list }) => {
                let param = parameter_from_list(&list).await?;
                execute_with_parameter(&*resource, &param, opts).await?;
            }
            Ok(SkimTarget::None) => (),
            Err(e) => return Err(e),
        }
    }
    Ok(())
}

pub async fn parameter_from_resource(
    parameter_resource_name: &str,
    opts: &Opts,
) -> Result<Option<String>> {
    for parameter_resource in all_resources() {
        if parameter_resource.name() == parameter_resource_name {
            let (_, selected_items) =
                resource_selector(parameter_resource.as_ref(), &None, opts).await?;
            for item in selected_items {
                return Ok(Some(item.output().to_string()));
            }
        }
    }

    Ok(None)
}

pub async fn parameter_from_list(
    selection: &(String, Vec<(String, String)>),
) -> Result<Option<String>> {
    let selected_items = list_selector(selection).await?;
    for item in selected_items {
        return Ok(Some(item.output().to_string()));
    }

    Ok(None)
}
pub fn tag_value<'a>(tags: &'a Yaml, name: &str) -> &'a Yaml {
    if let Yaml::Array(array) = tags {
        for tag in array {
            if Yaml::String(name.to_string()) == tag["key"] {
                return &tag["value"];
            }
        }
    }
    &Yaml::BadValue
}

pub fn next_token(yaml: &Yaml) -> Option<String> {
    match &yaml["next_token"] {
        Yaml::String(token) => Some(token.to_owned()),
        _ => None,
    }
}

fn print_selected_items(opts: &Opts, selected_items: &Vec<Arc<dyn SkimItem>>) {
    for (i, item) in selected_items.iter().enumerate() {
        if 0 == i {
            print!("{}", item.output());
        } else {
            print!("{}{}", opts.delimiter(), item.output());
        }
    }
}

pub async fn execute(resource: &dyn AwsResource, opts: &Opts) -> Result<()> {
    match resource.without_param(opts) {
        SkimTarget::ExecuteThis { parameter } => {
            execute_with_parameter(&*resource, &parameter, &opts).await?
        }
        SkimTarget::ParameterFromResource { resource_name } => {
            if let Some(param) = parameter_from_resource(&resource_name, opts).await? {
                execute_with_parameter(&*resource, &Some(param), opts).await?;
            }
        }
        SkimTarget::ParameterFromList { list } => {
            let param = parameter_from_list(&list).await?;
            execute_with_parameter(&*resource, &param, opts).await?;
        }
        SkimTarget::None => (),
    }
    Ok(())
}

async fn execute_with_parameter(
    resource: &dyn AwsResource,
    parameter: &Option<String>,
    opts: &Opts,
) -> Result<()> {
    let (yaml_list, selected_items) = resource_selector(resource, parameter, opts).await?;

    print_selected_items(opts, &selected_items);

    if opts.export {
        export_selected_items(resource, &yaml_list, &selected_items)?;
    }
    Ok(())
}

pub async fn resource_selector(
    resource: &dyn AwsResource,
    parameter: &Option<String>,
    opts: &Opts,
) -> Result<(Vec<Yaml>, Vec<Arc<dyn SkimItem>>)> {
    let mut info = vec![BarInfo::Resource(ResourceInfo {
        service_name: resource.service_name().to_owned(),
        command_name: resource.command_name().to_owned(),
    })];

    if !opts.cache {
        info.push(BarInfo::Fetch(fetch_loop(resource, parameter, opts).await?));
    } else {
        info.push(BarInfo::Cache(CacheInfo {
            opt_str: opts.colored_string(),
        }));
    }

    let yaml_list = read_yaml(resource);
    let selected_items = crate::skimmer::resources::skim(resource, &yaml_list, &info)?;

    Ok((yaml_list, selected_items))
}

pub async fn list_selector(
    list: &(String, Vec<(String, String)>),
) -> Result<Vec<Arc<dyn SkimItem>>> {
    Ok(crate::skimmer::list::skim(list)?)
}

async fn fetch_loop(
    resource: &dyn AwsResource,
    parameter: &Option<String>,
    opts: &Opts,
) -> Result<FetchInfo> {
    let mut info: FetchInfo = FetchInfo::new(opts)?;

    let mut result = vec![];
    let mut next_token: Option<String> = None;
    for i in 0..opts.request_count() {
        info.request_count = i + 1;
        let (mut list, token) = fetch(resource, parameter, opts, next_token.clone()).await?;

        result.append(&mut list);
        next_token = token;
        if next_token.is_none() {
            info.fetch_all = true;
            break;
        }
    }
    info.resource_count = result.len();
    file::store_yaml_list(&Yaml::Array(result), resource)?;
    Ok(info)
}

async fn fetch(
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

fn export_selected_items(
    resource: &dyn AwsResource,
    yaml_list: &Vec<Yaml>,
    selected_items: &Vec<Arc<dyn SkimItem>>,
) -> Result<()> {
    for (i, item) in selected_items.iter().enumerate() {
        for yaml in yaml_list {
            let item_name = &item.output().to_string();
            if resource.equal(yaml, item_name) {
                file::store_yaml(
                    yaml,
                    &format!(
                        "{}-{}-{}",
                        resource.command_name(),
                        resource.resource_type_name(),
                        i + 1
                    ),
                )?;
            }
        }
    }
    Ok(())
}

fn read_yaml(resource: &dyn AwsResource) -> Vec<Yaml> {
    let mut yaml_list: Vec<Yaml> = vec![];
    match file::restore_yaml(resource) {
        Some(arr) => {
            for yaml in &arr {
                yaml_list.push(yaml.clone());
            }
        }
        None => (),
    }
    yaml_list
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
