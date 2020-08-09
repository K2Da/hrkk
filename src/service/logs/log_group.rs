use crate::service::prelude::*;

#[derive(Serialize)]
pub(crate) struct Resource {
    info: Info,
}

pub(crate) fn new() -> Resource {
    Resource {
        info: Info {
            sub_command: Some(SubCommand::Logs {
                command: Logs::LogGroup,
            }),
            key_attribute: Some("log_group_name"),
            service_name: "logs",
            resource_type_name: "log_group",
            header: vec!["name"],
            list_api: ListApi {
                format: ListFormat::Json(ListJson {
                    method: JsonListMethod::Post {
                        target: "Logs_20140328.DescribeLogGroups",
                    },
                    service_name: "logs",
                    json: json!({}),
                    limit: Some(Limit {
                        name: "limit",
                        max: 50,
                    }),
                    token_name: Some("nextToken"),
                    parameter_name: None,
                }),
                document: DocumentUrl(
                    "AmazonCloudWatchLogs/latest/APIReference/API_DescribeLogGroups.html",
                ),
            },
            get_api: None,
            resource_url: Some(Regional(
                "cloudwatch/home?#logsV2:log-groups/log-group/{log_group_name}",
            )),
        },
    }
}

impl AwsResource for Resource {
    fn info(&self) -> &Info {
        &self.info
    }

    fn list_and_next_token(&self, yaml: &Yaml) -> (ResourceList, Option<String>) {
        (
            make_resource_list(self, &yaml["log_groups"]),
            next_token(&yaml, Some("next_token")),
        )
    }

    fn line(&self, list: &Yaml, _get: &Option<Yaml>) -> Vec<String> {
        vec![raw(&list["log_group_name"])]
    }

    fn detail(&self, list: &Yaml, get: &Option<Yaml>, region: &str) -> Section {
        Section::new(list)
            .yaml_name("log_group_name")
            .resource_url(self.console_url(list, get, region))
            .raw("arn")
            .time("creation_time")
            .raw("metric_filter_count")
            .byte("stored_bytes")
    }

    fn url_params(&self, list: &Yaml, _get: &Option<Yaml>) -> Option<Vec<ParamSet>> {
        Some(vec![("log_group_name", raw(&list["log_group_name"]), true)])
    }
}
