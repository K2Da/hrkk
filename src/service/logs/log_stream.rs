use crate::service::prelude::*;
use crate::service::resource_by_name;

#[derive(Serialize)]
pub struct Resource {
    info: Info,
}

pub fn new() -> Resource {
    Resource {
        info: Info {
            key_attribute: "log_stream_name",
            service_name: "logs",
            resource_type_name: "log_stream",
            api_type: ApiType::Json {
                service_name: "logs",
                target: "Logs_20140328.DescribeLogStreams",
                json: json!({ "descending": Some(true), "orderBy": Some("LastEventTime".to_owned()) }),
                limit_name: "limit",
                token_name: "nextToken",
                parameter_name: Some("logGroupName"),
            },

            document_url:
            "https://docs.aws.amazon.com/AmazonCloudWatchLogs/latest/APIReference/API_DescribeLogStreams.html",
            max_limit: 50,
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

    fn without_param(&self, opts: &Opts) -> ExecuteTarget {
        if opts.cache {
            ExecuteTarget::ExecuteThis { parameter: None }
        } else {
            ExecuteTarget::ParameterFromResource {
                param_resource: resource_by_name("logs_log_group"),
            }
        }
    }

    fn make_vec(&self, yaml: &Yaml) -> (Vec<Yaml>, Option<String>) {
        json_helper::make_vec(&yaml, "log_streams")
    }

    fn header(&self) -> Vec<&'static str> {
        vec!["time", "name"]
    }

    fn header_width(&self) -> Vec<Constraint> {
        vec![Constraint::Length(20), Constraint::Min(0)]
    }

    fn line(&self, item: &Yaml) -> Vec<String> {
        vec![
            show::span(
                &item["first_event_timestamp"],
                &item["last_event_timestamp"],
            ),
            show::raw(&item["log_stream_name"]),
        ]
    }

    fn detail(&self, yaml: &Yaml) -> crate::show::Section {
        crate::show::Section::new(&yaml)
            .yaml_name("log_stream_name")
            .raw("arn", "arn")
            .raw("creation time", "creation_time")
            .span(
                "event between",
                ("first_event_timestamp", "last_event_timestamp"),
            )
            .time("last ingestion", "last_ingestion_time")
            .raw("upload sequence token", "upload_sequence_token")
            .section(
                crate::show::Section::new(&yaml)
                    .string_name("path")
                    .string_attributes(
                        crate::show::raw(&yaml["log_stream_name"])
                            .split("/")
                            .enumerate()
                            .map(|(i, o)| (format!("{}", i + 1), o.to_owned()))
                            .collect(),
                    ),
            )
    }
}
