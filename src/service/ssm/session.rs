use crate::service::prelude::*;

#[derive(Serialize)]
pub struct Resource {
    info: Info,
}

pub fn new() -> Resource {
    Resource {
        info: Info {
            key_attribute: "session_id",
            service_name: "ssm",
            resource_type_name: "session",
            api_type: ApiType::Json {
                service_name: "ssm",
                target: "AmazonSSM.DescribeSessions",
                json: json!({}),
                limit_name: "MaxResults",
                token_name: "NextToken",
                parameter_name: Some("State"),
            },
            document_url: "https://docs.aws.amazon.com/systems-manager/latest/APIReference/API_DescribeSessions.html",
            max_limit: 200,
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
        if let SubCommand::Ssm {
            command: SsmCommand::Session { state },
        } = sub_command
        {
            match state {
                Some(text) => {
                    let text = text.to_pascal_case();
                    if text == "Active" || text == "History" {
                        Ok(ExecuteTarget::ExecuteThis {
                            parameter: Some(text),
                        })
                    } else {
                        Err(ParameterError(format!(
                            "state should be Active or History, but {}",
                            text
                        )))
                    }
                }
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
            ExecuteTarget::ParameterFromList {
                option_name: "State".to_string(),
                option_list: vec!["Active".to_string(), "History".to_string()],
            }
        }
    }

    fn make_vec(&self, yaml: &Yaml) -> (Vec<Yaml>, Option<String>) {
        json_helper::make_vec(&yaml, "sessions")
    }

    fn header(&self) -> Vec<&'static str> {
        vec!["id", "target", "date"]
    }

    fn header_width(&self) -> Vec<Constraint> {
        vec![
            Constraint::Length(20),
            Constraint::Length(20),
            Constraint::Min(0),
        ]
    }

    fn line(&self, item: &Yaml) -> Vec<String> {
        vec![
            show::raw(&item["session_id"]),
            show::raw(&item["target"]),
            show::span(&item["start_date"], &item["end_date"]),
        ]
    }

    fn detail(&self, yaml: &Yaml) -> crate::show::Section {
        crate::show::Section::new(&yaml)
            .yaml_name("session_id")
            .raw("owner", "owner")
            .raw("target", "target")
            .raw("status", "status")
            .span("date", ("start_date", "end_date"))
            .section(
                crate::show::Section::new(&yaml["output_url"])
                    .string_name("output url")
                    .raw("cloudwatch", "cloud_watch_output_url")
                    .raw("s3", "s3_output_url"),
            )
    }
}
