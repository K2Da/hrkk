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
            api_type: ApiType::Json(JsonApi {
                service_name: "logs",
                target: "Logs_20140328.DescribeLogGroups",
                json: json!({}),
                limit_name: "limit",
                token_name: "nextToken",
                parameter_name: None,
                max_limit: 50,
            }),

            document_url:
            "https://docs.aws.amazon.com/AmazonCloudWatchLogs/latest/APIReference/API_DescribeLogGroups.html",
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

    fn make_vec(&self, yaml: &Yaml) -> (Vec<Yaml>, Option<String>) {
        json_helper::make_vec(&yaml, "log_groups")
    }

    fn header(&self) -> Vec<&'static str> {
        vec!["name"]
    }

    fn line(&self, item: &Yaml) -> Vec<String> {
        vec![show::raw(&item["log_group_name"])]
    }

    fn detail(&self, yaml: &Yaml) -> crate::show::Section {
        crate::show::Section::new(&yaml)
            .yaml_name("log_group_name")
            .raw("arn", "arn")
            .time("creation time", "creation_time")
            .raw("metric filter count", "metric_filter_count")
            .byte("stored bytes", "stored_bytes")
    }
}
