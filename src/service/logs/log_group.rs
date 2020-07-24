use crate::service::prelude::*;

#[derive(Serialize)]
pub(crate) struct Resource {
    info: Info,
}

pub(crate) fn new() -> Resource {
    Resource {
        info: Info {
            key_attribute: "log_group_name",
            service_name: "logs",
            resource_type_name: "log_group",
            list_api: ListApi::Json(JsonListApi {
                method: JsonListMethod::Post {
                    target: "Logs_20140328.DescribeLogGroups",
                },
                service_name: "logs",
                json: json!({}),
                limit_name: "limit",
                token_name: "nextToken",
                parameter_name: None,
                max_limit: 50,
            }),
            get_api: None,

            list_api_document_url:
            "https://docs.aws.amazon.com/AmazonCloudWatchLogs/latest/APIReference/API_DescribeLogGroups.html",
            get_api_document_url: None,
            resource_url: Some(ResourceUrl::Regional("cloudwatch/home?#logsV2:log-groups/log-group/{log_group_name}"))
        },
    }
}

impl AwsResource for Resource {
    fn info(&self) -> &Info {
        &self.info
    }

    fn matching_sub_command(&self) -> Option<SubCommand> {
        Some(SubCommand::Logs {
            command: LogsCommand::LogGroup,
        })
    }

    fn make_vec(&self, yaml: &Yaml) -> (ResourceList, Option<String>) {
        make_vec(self, &yaml["log_groups"], "next_token")
    }

    fn header(&self) -> Vec<&'static str> {
        vec!["name"]
    }

    fn line(&self, list: &Yaml, _get: &Option<Yaml>) -> Vec<String> {
        vec![show::raw(&list["log_group_name"])]
    }

    fn detail(&self, list: &Yaml, get: &Option<Yaml>, region: &str) -> Section {
        Section::new(&list)
            .yaml_name("log_group_name")
            .resource_url(self.console_url(list, get, region))
            .raw("arn")
            .time("creation_time")
            .raw("metric_filter_count")
            .byte("stored_bytes")
    }

    fn url_params(&self, list: &Yaml, _get: &Option<Yaml>) -> Option<Vec<(&'static str, String)>> {
        Some(vec![("log_group_name", show::raw(&list["log_group_name"]))])
    }
}
