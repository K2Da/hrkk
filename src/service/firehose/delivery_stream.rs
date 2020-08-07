use crate::service::prelude::*;

#[derive(Serialize)]
pub(crate) struct Resource {
    info: Info,
}

pub(crate) fn new() -> Resource {
    Resource {
        info: Info {
            sub_command: Some(SubCommand::Firehose { command: Firehose::DeliveryStream }),
            key_attribute: Some("delivery_stream_name"),
            service_name: "firehose",
            resource_type_name: "delivery_stream",
            header: vec!["name", "create timestamp"],
            list_api: ListApi {
                format: ListFormat::Json(ListJson {
                    method: JsonListMethod::Post { target: "Firehose_20150804.ListDeliveryStreams", },
                    service_name: "firehose",
                    json: json!({}),
                    limit: Some(Limit { name: "Limit", max: 10000 }),
                    token_name: Some("ExclusiveStartDeliveryStreamName"),
                    parameter_name: None,
                }),
                document: DocumentUrl("firehose/latest/APIReference/API_ListDeliveryStreams.html"),
            },
            get_api: Some(GetApi {
                param_path: vec![],
                format: GetFormat::Json(GetJson {
                    method: Method::Post,
                    path: ("/", None),
                    service_name: "firehose",
                    target: Some("Firehose_20150804.DescribeDeliveryStream"),
                    parameter_name: Some("DeliveryStreamName"),
                }),
                document: "https://docs.aws.amazon.com/firehose/latest/APIReference/API_DescribeDeliveryStream.html"
            }),
            resource_url: Some(Regional("firehose/home?#/details/{delivery_stream_name}")),
        },
    }
}

impl AwsResource for Resource {
    fn info(&self) -> &Info {
        &self.info
    }

    fn list_and_next_token(&self, yaml: &Yaml) -> (ResourceList, Option<String>) {
        let rl = make_resource_list(self, &yaml["delivery_stream_names"]);
        let last_stream_name = match rl.last() {
            Some(last) => last.0.first().map(|s| s.clone()),
            None => None,
        };
        (rl, last_stream_name)
    }

    fn line(&self, list: &Yaml, get: &Option<Yaml>) -> Vec<String> {
        match get {
            Some(get) => {
                let root = &get["delivery_stream_description"];
                vec![
                    raw(&root["delivery_stream_name"]),
                    time(&root["create_timestamp"]),
                ]
            }
            None => vec![raw(&list), "".to_string()],
        }
    }

    fn detail(&self, list: &Yaml, get: &Option<Yaml>, region: &str) -> Section {
        match get {
            None => Section::new(list),
            Some(yaml) => {
                let root = &yaml["delivery_stream_description"];
                Section::new(root)
                    .yaml_name("delivery_stream_name")
                    .resource_url(self.console_url(list, get, region))
                    .raw("delivery_stream_status")
                    .raw("delivery_stream_arn")
                    .time("create_timestamp")
                    .time("last_update_timestamp")
                    .raw("version_id")
            }
        }
    }

    fn url_params(&self, _list: &Yaml, get: &Option<Yaml>) -> Option<Vec<(&'static str, String)>> {
        if let Some(yaml) = get {
            Some(vec![(
                "delivery_stream_name",
                raw(&yaml["delivery_stream_description"]["delivery_stream_name"]),
            )])
        } else {
            None
        }
    }
}
