use crate::service::prelude::*;

#[derive(Serialize)]
pub(crate) struct Resource {
    info: Info,
}

pub(crate) fn new() -> Resource {
    Resource {
        info: Info {
            sub_command: Some(SubCommand::Kinesis { command: Kinesis::Stream }),
            key_attribute: Some("stream_name"),
            service_name: "kinesis",
            resource_type_name: "stream",
            header: vec!["name", "creation timestamp"],
            list_api: ListApi {
                format: ListFormat::Json(ListJson {
                    method: JsonListMethod::Post { target: "Kinesis_20131202.ListStreams", },
                    service_name: "kinesis",
                    json: json!({}),
                    limit: Some(Limit { name: "Limit", max: 10000 }),
                    token_name: Some("ExclusiveStartStreamName"),
                    parameter_name: None,
                }),
                document: "https://docs.aws.amazon.com/kinesis/latest/APIReference/API_ListStreams.html",
            },
            get_api: Some(GetApi {
                param_path: vec![],
                format: GetFormat::Json(GetJson {
                    method: Method::Post,
                    path: ("/", None),
                    service_name: "kinesis",
                    target: Some("Kinesis_20131202.DescribeStream"),
                    parameter_name: Some("StreamName"),
                }),
                document: "https://docs.aws.amazon.com/kinesis/latest/APIReference/API_DescribeStream.html",
            }),
            resource_url: Some(Regional("kinesis/home?#/streams/details/{stream_name}/details")),
        },
    }
}

impl AwsResource for Resource {
    fn info(&self) -> &Info {
        &self.info
    }

    fn list_and_next_token(&self, yaml: &Yaml) -> (ResourceList, Option<String>) {
        let rl = make_resource_list(self, &yaml["stream_names"]);
        let last_stream_name = match rl.last() {
            Some(last) => last.0.first().map(|s| s.clone()),
            None => None,
        };
        (rl, last_stream_name)
    }

    fn line(&self, list: &Yaml, get: &Option<Yaml>) -> Vec<String> {
        match get {
            Some(get) => {
                let root = &get["stream_description"];
                vec![
                    show::raw(&root["stream_name"]),
                    show::time(&root["stream_creation_timestamp"]),
                ]
            }
            None => vec![show::raw(&list), "".to_string()],
        }
    }

    fn detail(&self, list: &Yaml, get: &Option<Yaml>, region: &str) -> Section {
        match get {
            None => Section::new(list),
            Some(yaml) => {
                let root = &yaml["stream_description"];
                Section::new(root)
                    .yaml_name("stream_name")
                    .resource_url(self.console_url(list, get, region))
                    .raw("stream_status")
                    .raw("stream_arn")
                    .time("stream_creation_timestamp")
                    .raw("encryption_type")
                    .raw("has_more_shards")
                    .raw("retention_period_hours")
            }
        }
    }

    fn url_params(&self, _list: &Yaml, get: &Option<Yaml>) -> Option<Vec<(&'static str, String)>> {
        if let Some(yaml) = get {
            Some(vec![(
                "stream_name",
                show::raw(&yaml["stream_description"]["stream_name"]),
            )])
        } else {
            None
        }
    }
}
