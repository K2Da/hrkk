use crate::service::prelude::*;

#[derive(Serialize)]
pub(crate) struct Resource {
    info: Info,
}

pub(crate) fn new() -> Resource {
    Resource {
        info: Info {
            key_attribute: "session_id",
            service_name: "ssm",
            resource_type_name: "session",
            list_api: ListApi::Json(JsonListApi {
                method: JsonListMethod::Post {
                    target: "AmazonSSM.DescribeSessions",
                },
                service_name: "ssm",
                json: json!({}),
                limit_name: "MaxResults",
                token_name: "NextToken",
                parameter_name: Some("State"),
                max_limit: 200,
            }),
            get_api: None,
            list_api_document_url: "https://docs.aws.amazon.com/systems-manager/latest/APIReference/API_DescribeSessions.html",
            get_api_document_url: None,
            resource_url: None,
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

    fn without_param(&self, _opts: &Opts) -> ExecuteTarget {
        ExecuteTarget::ParameterFromList {
            option_name: "State".to_string(),
            option_list: vec!["Active".to_string(), "History".to_string()],
        }
    }

    fn make_vec(&self, yaml: &Yaml) -> (ResourceList, Option<String>) {
        make_vec(self, &yaml["sessions"], "next_token")
    }

    fn header(&self) -> Vec<&'static str> {
        vec!["id", "target", "date"]
    }

    fn line(&self, list: &Yaml, _get: &Option<Yaml>) -> Vec<String> {
        vec![
            show::raw(&list["session_id"]),
            show::raw(&list["target"]),
            show::span(&list["start_date"], &list["end_date"]),
        ]
    }

    fn detail(&self, yaml: &Yaml, _get_yaml: &Option<Yaml>, _region: &str) -> Section {
        Section::new(&yaml)
            .yaml_name("session_id")
            .raw("owner")
            .raw("target")
            .raw("status")
            .span("date", ("start_date", "end_date"))
            .section(
                Section::new(&yaml["output_url"])
                    .string_name("output url")
                    .raw1("cloudwatch", "cloud_watch_output_url")
                    .raw1("s3", "s3_output_url"),
            )
    }
}
