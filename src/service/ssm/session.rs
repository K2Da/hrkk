use crate::service::prelude::*;

#[derive(Serialize)]
pub(crate) struct Resource {
    info: Info,
}

pub(crate) fn new() -> Resource {
    Resource {
        info: Info {
            sub_command: None,
            key_attribute: Some("session_id"),
            service_name: "ssm",
            resource_type_name: "session",
            header: vec!["id", "target", "date"],
            list_api: ListApi {
                format: ListFormat::Json(ListJson {
                    method: JsonListMethod::Post {
                        target: "AmazonSSM.DescribeSessions",
                    },
                    service_name: "ssm",
                    json: json!({}),
                    limit: Some(Limit { name: "MaxResults", max: 200 }),
                    token_name: Some("NextToken"),
                    parameter_name: Some("State"),
                }),
                document: "https://docs.aws.amazon.com/systems-manager/latest/APIReference/API_DescribeSessions.html",
            },
            get_api: None,
            resource_url: None,
        },
    }
}

impl AwsResource for Resource {
    fn info(&self) -> &Info {
        &self.info
    }

    fn take_command(&self, sub_command: &SubCommand, opts: &Opts) -> Result<ExecuteTarget> {
        if let SubCommand::Ssm {
            command: Ssm::Session { state },
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

    fn list_and_next_token(&self, yaml: &Yaml) -> (ResourceList, Option<String>) {
        (
            make_resource_list(self, &yaml["sessions"]),
            next_token(&yaml, Some("next_token")),
        )
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
                    .raw_n("cloudwatch", &["cloud_watch_output_url"])
                    .raw_n("s3", &["s3_output_url"]),
            )
    }
}
