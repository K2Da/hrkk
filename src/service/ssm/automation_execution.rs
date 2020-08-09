use crate::service::prelude::*;

#[derive(Serialize)]
pub(crate) struct Resource {
    info: Info,
}

pub(crate) fn new() -> Resource {
    Resource {
        info: Info {
            sub_command: Some(SubCommand::Ssm {
                command: Ssm::AutomationExecution,
            }),
            key_attribute: Some("automation_execution_id"),
            service_name: "ssm",
            resource_type_name: "automation_execution",
            header: vec!["status", "name", "time", "end at"],
            list_api: ListApi {
                format: ListFormat::Json(ListJson {
                    method: JsonListMethod::Post {
                        target: "AmazonSSM.DescribeAutomationExecutions",
                    },
                    service_name: "ssm",
                    json: json!({}),
                    limit: Some(Limit {
                        name: "MaxResults",
                        max: 50,
                    }),
                    token_name: Some("NextToken"),
                    parameter_name: None,
                }),
                document: DocumentUrl(
                    "systems-manager/latest/APIReference/API_DescribeAutomationExecutions.html",
                ),
            },
            get_api: None,
            resource_url: Some(Regional(
                "systems-manager/automation/execution/{execution_id}",
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
            make_resource_list(self, &yaml["automation_execution_metadata_list"]),
            next_token(&yaml, Some("next_token")),
        )
    }

    fn line(&self, list: &Yaml, _get: &Option<Yaml>) -> Vec<String> {
        vec![
            raw(&list["automation_execution_status"]),
            raw(&list["document_name"]),
            duration(&list["execution_start_time"], &list["execution_end_time"]),
            time(&list["execution_end_time"]),
        ]
    }

    fn detail(&self, list: &Yaml, get: &Option<Yaml>, region: &str) -> Section {
        Section::new(list)
            .yaml_name("automation_execution_id")
            .resource_url(self.console_url(list, get, region))
            .raw_n("status", &["automation_execution_status"])
            .duration(
                "execution time",
                ("execution_start_time", "execution_end_time"),
            )
            .span("from to", ("execution_start_time", "execution_end_time"))
            .raw("automation_type")
            .raw("document_name")
            .raw("document_version")
            .raw("executed_by")
            .raw("mode")
    }

    fn url_params(&self, list: &Yaml, _get: &Option<Yaml>) -> Option<Vec<ParamSet>> {
        Some(vec![(
            "execution_id",
            raw(&list["automation_execution_id"]),
            true,
        )])
    }
}
