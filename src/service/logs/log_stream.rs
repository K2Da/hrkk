use crate::service::prelude::*;

#[derive(Serialize)]
pub(crate) struct Resource {
    info: Info,
}

pub(crate) fn new() -> Resource {
    Resource {
        info: Info {
            sub_command: None,
            key_attribute: Some("log_stream_name"),
            service_name: "logs",
            resource_type_name: "log_stream",
            header: vec!["time", "name"],
            list_api: ListApi {
                format: ListFormat::Json(ListJson {
                    method: JsonListMethod::Post {
                        target: "Logs_20140328.DescribeLogStreams",
                    },
                    service_name: "logs",
                    json: json!({ "descending": Some(true), "orderBy": Some("LastEventTime".to_owned()) }),
                    limit: Some(Limit { name: "limit", max: 50 }),
                    token_name: Some("nextToken"),
                    parameter_name: Some("logGroupName"),
                }),
                document: "https://docs.aws.amazon.com/AmazonCloudWatchLogs/latest/APIReference/API_DescribeLogStreams.html",
            },
            get_api: None,
            resource_url: Some(Regional("cloudwatch/home?#logsV2:log-groups/log-group/{group_name}/log-events/{stream_name}")),
        },
    }
}

impl AwsResource for Resource {
    fn info(&self) -> &Info {
        &self.info
    }

    fn take_command(&self, sub_command: &SubCommand, opts: &Opts) -> Result<ExecuteTarget> {
        if let SubCommand::Logs {
            command: Logs::LogStream { log_group_name },
        } = sub_command
        {
            match log_group_name {
                Some(text) => Ok(ExecuteTarget::ExecuteThis {
                    parameter: Some(text.clone()),
                }),
                None => Ok(self.without_param(opts)),
            }
        } else {
            Ok(ExecuteTarget::Null)
        }
    }

    fn without_param(&self, _opts: &Opts) -> ExecuteTarget {
        ExecuteTarget::ParameterFromResource {
            param_resource: resource_by_name("logs_log_group"),
        }
    }

    fn list_and_next_token(&self, yaml: &Yaml) -> (ResourceList, Option<String>) {
        (
            make_resource_list(self, &yaml["log_streams"]),
            next_token(&yaml, Some("next_token")),
        )
    }

    fn line(&self, list: &Yaml, _get: &Option<Yaml>) -> Vec<String> {
        vec![
            span(
                &list["first_event_timestamp"],
                &list["last_event_timestamp"],
            ),
            raw(&list["log_stream_name"]),
        ]
    }

    fn detail(&self, list: &Yaml, get: &Option<Yaml>, region: &str) -> Section {
        Section::new(list)
            .yaml_name("log_stream_name")
            .resource_url(self.console_url(list, get, region))
            .raw("arn")
            .time("creation_time")
            .span(
                "event between",
                ("first_event_timestamp", "last_event_timestamp"),
            )
            .time("last_ingestion_time")
            .raw("upload_sequence_token")
            .section(
                Section::new(list).string_name("path").string_attributes(
                    raw(&list["log_stream_name"])
                        .split("/")
                        .enumerate()
                        .map(|(i, o)| (format!("{}", i + 1), o.to_owned()))
                        .collect(),
                ),
            )
    }

    fn url_params(&self, list: &Yaml, _get: &Option<Yaml>) -> Option<Vec<(&'static str, String)>> {
        Some(vec![
            (
                "group_name",
                raw(&list["arn"]).split(":").collect::<Vec<&str>>()[6].to_owned(),
            ),
            ("stream_name", raw(&list["log_stream_name"])),
        ])
    }
}
