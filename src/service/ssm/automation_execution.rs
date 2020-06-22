use crate::service::prelude::*;

#[derive(Serialize)]
pub struct Resource {
    info: Info,
}

pub fn new() -> Resource {
    Resource {
        info: Info {
            key_attribute: "AutomationExecutionId",
            service_name: "ssm",
            resource_type_name: "automation_execution",
            api_type: ApiType::Json {
                service_name: "ssm",
                target: "AmazonSSM.DescribeAutomationExecutions",
                json: json!({}),
                limit_name: "MaxResults",
                token_name: "NextToken",
                parameter_name: None,
            },

            document_url: "https://docs.aws.amazon.com/systems-manager/latest/APIReference/API_DescribeAutomationExecutions.html",
            max_limit: 50,
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

    fn make_vec(&self, yaml: &Yaml) -> (Vec<Yaml>, Option<String>) {
        json_helper::make_vec(&yaml, "automation_execution_metadata_list")
    }

    fn header(&self) -> Vec<&'static str> {
        vec!["status", "name", "time", "end at"]
    }

    fn header_width(&self) -> Vec<Constraint> {
        vec![
            Constraint::Length(10),
            Constraint::Min(22),
            Constraint::Length(8),
            Constraint::Length(18),
        ]
    }

    fn line(&self, item: &Yaml) -> Vec<String> {
        vec![
            show::raw(&item["automation_execution_status"]),
            show::raw(&item["document_name"]),
            show::duration(&item["execution_start_time"], &item["execution_end_time"]),
            show::time(&item["execution_end_time"]),
        ]
    }

    fn detail(&self, yaml: &Yaml) -> crate::show::Section {
        crate::show::Section::new(&yaml)
            .yaml_name("automation_execution_id")
            .raw("status", "automation_execution_status")
            .span(
                "execution time",
                ("execution_start_time", "execution_end_time"),
            )
            .raw("type", "automation_type")
            .raw("document", "document_name")
            .raw("document version", "document_version")
            .raw("executed by", "executed_by")
            .raw("mode", "mode")
    }
}
