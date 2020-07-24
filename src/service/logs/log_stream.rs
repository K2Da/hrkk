use crate::service::prelude::*;

#[derive(Serialize)]
pub(crate) struct Resource {
    info: Info,
}

pub(crate) fn new() -> Resource {
    Resource {
        info: Info {
            key_attribute: "log_stream_name",
            service_name: "logs",
            resource_type_name: "log_stream",
            list_api: ListApi::Json(JsonListApi {
                method: JsonListMethod::Post {
                    target: "Logs_20140328.DescribeLogStreams",
                },
                service_name: "logs",
                json: json!({ "descending": Some(true), "orderBy": Some("LastEventTime".to_owned()) }),
                limit_name: "limit",
                token_name: "nextToken",
                parameter_name: Some("logGroupName"),
                max_limit: 50,
            }),
            get_api: None,
            list_api_document_url:
            "https://docs.aws.amazon.com/AmazonCloudWatchLogs/latest/APIReference/API_DescribeLogStreams.html",
            get_api_document_url: None,
            resource_url: Some(ResourceUrl::Regional("cloudwatch/home?#logsV2:log-groups/log-group/{group_name}/log-events/{stream_name}")),
        },
    }
}

impl AwsResource for Resource {
    fn info(&self) -> &Info {
        &self.info
    }

    fn matching_sub_command(&self) -> Option<SubCommand> {
        None
    }

    fn take_command(&self, sub_command: &SubCommand, opts: &Opts) -> Result<ExecuteTarget> {
        if let SubCommand::Logs {
            command: LogsCommand::LogStream { log_group_name },
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

    fn make_vec(&self, yaml: &Yaml) -> (ResourceList, Option<String>) {
        make_vec(self, &yaml["log_streams"], "next_token")
    }

    fn header(&self) -> Vec<&'static str> {
        vec!["time", "name"]
    }

    fn line(&self, list: &Yaml, _get: &Option<Yaml>) -> Vec<String> {
        vec![
            show::span(
                &list["first_event_timestamp"],
                &list["last_event_timestamp"],
            ),
            show::raw(&list["log_stream_name"]),
        ]
    }

    fn detail(&self, list: &Yaml, get: &Option<Yaml>, region: &str) -> Section {
        Section::new(&list)
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
                Section::new(&list).string_name("path").string_attributes(
                    show::raw(&list["log_stream_name"])
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
                show::raw(&list["arn"]).split(":").collect::<Vec<&str>>()[6].to_owned(),
            ),
            ("stream_name", show::raw(&list["log_stream_name"])),
        ])
    }
}
