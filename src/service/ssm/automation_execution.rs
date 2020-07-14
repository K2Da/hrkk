use crate::service::prelude::*;

#[derive(Serialize)]
pub(crate) struct Resource {
    info: Info,
}

pub(crate) fn new() -> Resource {
    Resource {
        info: Info {
            key_attribute: "AutomationExecutionId",
            service_name: "ssm",
            resource_type_name: "automation_execution",
            list_api: ListApi::Json(JsonListApi {
                method: JsonListMethod::Post {
                    target: "AmazonSSM.DescribeAutomationExecutions",
                },
                service_name: "ssm",
                json: json!({}),
                limit_name: "MaxResults",
                token_name: "NextToken",
                parameter_name: None,
                max_limit: 50,
            }),
            get_api: None,

            list_api_document_url: "https://docs.aws.amazon.com/systems-manager/latest/APIReference/API_DescribeAutomationExecutions.html",
            get_api_document_url: None,
            resource_url: Some(
                "systems-manager/automation/execution/{execution_id}"
            ),
        },
    }
}

impl AwsResource for Resource {
    fn info(&self) -> &Info {
        &self.info
    }

    fn matching_sub_command(&self) -> Option<SubCommand> {
        Some(SubCommand::Ssm {
            command: SsmCommand::AutomationExecution,
        })
    }

    fn make_vec(&self, yaml: &Yaml) -> (ResourceList, Option<String>) {
        make_vec(self, &yaml["automation_execution_metadata_list"])
    }

    fn header(&self) -> Vec<&'static str> {
        vec!["status", "name", "time", "end at"]
    }

    fn line(&self, list: &Yaml, _get: &Option<Yaml>) -> Vec<String> {
        vec![
            show::raw(&list["automation_execution_status"]),
            show::raw(&list["document_name"]),
            show::duration(&list["execution_start_time"], &list["execution_end_time"]),
            show::time(&list["execution_end_time"]),
        ]
    }

    fn detail(&self, list: &Yaml, get: &Option<Yaml>, region: &str) -> Section {
        Section::new(&list)
            .yaml_name("automation_execution_id")
            .resource_url(self.console_url(list, get, region))
            .raw1("status", "automation_execution_status")
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

    fn url_params(&self, list: &Yaml, _get: &Option<Yaml>) -> Option<Vec<(&'static str, String)>> {
        Some(vec![(
            "execution_id",
            show::raw(&list["automation_execution_id"]),
        )])
    }
}
