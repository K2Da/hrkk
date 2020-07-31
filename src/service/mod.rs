use crate::error::Error::*;
use crate::error::Result;
use crate::opts::{Opts, SubCommand};
use linked_hash_map::LinkedHashMap;
use serde::Serialize;
use serde_json::{Map, Value};
use yaml_rust::Yaml;

pub(crate) mod acm;
pub(crate) mod athena;
pub(crate) mod autoscaling;
pub(crate) mod cloudformation;
pub(crate) mod cloudfront;
pub(crate) mod cloudwatch;
pub(crate) mod ec2;
pub(crate) mod elasticache;
pub(crate) mod elb;
pub(crate) mod es;
pub(crate) mod firehose;
pub(crate) mod kinesis;
pub(crate) mod lambda;
pub(crate) mod logs;
pub(crate) mod prelude;
pub(crate) mod rds;
pub(crate) mod route53;
pub(crate) mod s3;
pub(crate) mod ssm;

pub(crate) type ResourceList = Vec<(Vec<String>, Yaml)>;

pub(crate) fn all_resources() -> Vec<Box<dyn AwsResource>> {
    vec![
        Box::new(acm::certificate::new()),
        Box::new(athena::query_execution::new()),
        Box::new(autoscaling::auto_scaling_group::new()),
        Box::new(cloudformation::stack::new()),
        Box::new(cloudfront::distribution::new()),
        Box::new(cloudwatch::alarm::new()),
        Box::new(cloudwatch::alarm_history::new()),
        Box::new(cloudwatch::dashboard::new()),
        Box::new(cloudwatch::metric::new()),
        Box::new(ec2::image::new()),
        Box::new(ec2::instance::new()),
        Box::new(ec2::launch_template::new()),
        Box::new(ec2::security_group::new()),
        Box::new(ec2::subnet::new()),
        Box::new(ec2::vpc::new()),
        Box::new(elasticache::cache_cluster::new()),
        Box::new(elb::load_balancer::new()),
        Box::new(es::domain::new()),
        Box::new(firehose::delivery_stream::new()),
        Box::new(kinesis::stream::new()),
        Box::new(lambda::function::new()),
        Box::new(logs::log_group::new()),
        Box::new(logs::log_stream::new()),
        Box::new(rds::db_instance::new()),
        Box::new(route53::hosted_zone::new()),
        Box::new(route53::resource_record_set::new()),
        Box::new(s3::bucket::new()),
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
    panic!("no resource with name = {}", name)
}

#[derive(Serialize)]
pub(crate) struct Info {
    key_attribute: Option<&'static str>,
    service_name: &'static str,
    resource_type_name: &'static str,
    pub(crate) list_api: ListApi,
    pub(crate) get_api: Option<GetApi>,
    pub(crate) resource_url: Option<ResourceUrl>,
}

#[derive(Serialize, Clone, Debug)]
pub(crate) enum ResourceUrl {
    Regional(&'static str),
    Global(&'static str),
}

#[derive(Serialize, Clone)]
pub(crate) struct ListApi {
    pub(crate) format: ListFormat,
    pub(crate) document: &'static str,
}

#[derive(Serialize, Clone)]
pub(crate) enum ListFormat {
    Xml(ListXml),
    Json(ListJson),
}

impl ListFormat {
    pub(crate) fn name(&self) -> String {
        match self {
            ListFormat::Xml(api) => format!("{} {:?}", api.path.0, api.params),
            ListFormat::Json(ListJson {
                method: JsonListMethod::Post { target },
                ..
            }) => target.to_string(),
            ListFormat::Json(ListJson {
                method: JsonListMethod::Get { path },
                ..
            }) => path.to_string(),
        }
    }

    pub(crate) fn parameter_name(&self) -> Option<&'static str> {
        match self {
            ListFormat::Xml(_) => None,
            ListFormat::Json(api) => api.parameter_name,
        }
    }
}

#[derive(Serialize, Clone)]
pub(crate) struct Limit {
    pub(crate) name: &'static str,
    pub(crate) max: i64,
}

#[derive(Serialize, Clone)]
pub(crate) struct ListXml {
    pub(crate) path: (&'static str, Option<&'static str>),
    pub(crate) method: Method,
    pub(crate) service_name: &'static str,
    pub(crate) iteration_tag: Vec<&'static str>,
    pub(crate) limit: Option<Limit>,
    pub(crate) token_name: &'static str,
    pub(crate) params: Vec<(&'static str, &'static str)>,
    pub(crate) region: Option<rusoto_core::Region>,
}

#[derive(Serialize, Clone)]
pub(crate) enum Method {
    Post,
    Get,
}

impl Method {
    pub fn to_str(&self) -> &'static str {
        match self {
            Method::Post => "POST",
            Method::Get => "GET",
        }
    }
}

#[derive(Serialize, Clone)]
pub(crate) struct ListJson {
    pub(crate) method: JsonListMethod,
    pub(crate) service_name: &'static str,
    pub(crate) json: serde_json::Value,
    pub(crate) limit: Option<Limit>,
    pub(crate) token_name: Option<&'static str>,
    pub(crate) parameter_name: Option<&'static str>,
}

#[derive(Serialize, Clone)]
pub(crate) enum JsonListMethod {
    Post { target: &'static str },
    Get { path: &'static str },
}

impl ListJson {
    pub(crate) fn json_map(&self) -> Result<Map<String, Value>> {
        if let Value::Object(map) = &self.json {
            return Ok(map.clone());
        } else {
            Err(SettingError("request json is not a map.".to_string()))
        }
    }
}

#[derive(Serialize, Clone)]
pub(crate) struct GetApi {
    pub(crate) format: GetFormat,
    pub(crate) document: &'static str,
}

#[derive(Serialize, Clone)]
pub(crate) enum GetFormat {
    Xml(GetXml),
    Json(GetJson),
}

impl GetFormat {
    pub(crate) fn name(&self) -> &'static str {
        match self {
            GetFormat::Xml(api) => api.action,
            GetFormat::Json(GetJson {
                target: Some(target),
                ..
            }) => target,
            _ => "-",
        }
    }
}

#[derive(Serialize, Clone)]
pub(crate) struct GetXml {
    pub(crate) service_name: &'static str,
    pub(crate) action: &'static str,
    pub(crate) version: &'static str,
    pub(crate) parameter_name: &'static str,
}

#[derive(Serialize, Clone)]
pub(crate) struct GetJson {
    pub(crate) method: Method,
    pub(crate) path: (&'static str, Option<&'static str>),
    pub(crate) service_name: &'static str,
    pub(crate) target: Option<&'static str>,
    pub(crate) parameter_name: Option<&'static str>,
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

    fn list_and_next_token(&self, yaml: &Yaml) -> (ResourceList, Option<String>);

    fn header(&self) -> Vec<&'static str>;

    fn line(&self, list: &Yaml, get: &Option<Yaml>) -> Vec<String>;

    fn detail(&self, list: &Yaml, get: &Option<Yaml>, region: &str) -> crate::show::Section;

    fn console_url(&self, list: &Yaml, get: &Option<Yaml>, region: &str) -> String {
        if let Some(resource_url) = &self.info().resource_url {
            let mut line = match resource_url {
                ResourceUrl::Regional(url) => {
                    format!("https://{}.console.aws.amazon.com/{}", region, url)
                }
                ResourceUrl::Global(url) => format!("https://console.aws.amazon.com/{}", url),
            };
            for (key, param) in self.url_params(list, get).unwrap_or(vec![]).iter() {
                line = line.replace(&("{".to_string() + key + "}"), &url_encoded(param));
            }
            return line;
        }
        "-".to_string()
    }

    fn url_params(&self, _list: &Yaml, _get: &Option<Yaml>) -> Option<Vec<(&'static str, String)>> {
        panic!("no url for this resource")
    }

    fn name(&self) -> String {
        format!("{}_{}", self.service_name(), self.resource_type_name())
    }

    fn equal(&self, yaml: &Yaml, key: &str) -> bool {
        self.resource_name(yaml) == key.to_string()
    }

    fn resource_name(&self, list: &Yaml) -> String {
        crate::show::raw(match self.info().key_attribute {
            Some(key_attribute) => &list[key_attribute],
            None => &list,
        })
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

    fn list_api(&self) -> ListFormat {
        self.info().list_api.format.clone()
    }

    fn get_api(&self) -> Option<GetFormat> {
        if let Some(api) = &self.info().get_api {
            return Some(api.format.clone());
        }
        None
    }

    fn has_get_api(&self) -> bool {
        self.info().get_api.is_some()
    }

    fn max_limit(&self) -> String {
        match &self.info().list_api.format {
            ListFormat::Xml(ListXml {
                limit: Some(limit), ..
            }) => format!("{}: {}", limit.name, limit.max),
            ListFormat::Json(ListJson {
                limit: Some(limit), ..
            }) => format!("{}: {}", limit.name, limit.max),
            _ => "-".to_owned(),
        }
    }

    fn has_resource_url(&self) -> bool {
        self.info().resource_url.is_some()
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

pub(crate) fn next_token(yaml: &Yaml, token_name: Option<&'static str>) -> Option<String> {
    if let Some(token_name) = token_name {
        if let Yaml::String(token) = &yaml[token_name] {
            return Some(token.to_owned());
        }
    }
    None
}

pub(crate) fn merge_yamls(yaml: &Yaml, get_yaml: &Yaml) -> Yaml {
    let mut merged = LinkedHashMap::new();
    merged.insert(Yaml::String("list".to_owned()), yaml.clone());
    merged.insert(Yaml::String("get".to_owned()), get_yaml.clone());
    Yaml::Hash(merged)
}

fn url_encoded(str: &str) -> String {
    match serde_urlencoded::to_string(&[("", str)]) {
        Ok(str) => str[1..].to_string(),
        Err(err) => format!("{:?}", err),
    }
}
