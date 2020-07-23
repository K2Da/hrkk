use crate::service::prelude::*;

#[derive(Serialize)]
pub(crate) struct Resource {
    info: Info,
}

pub(crate) fn new() -> Resource {
    Resource {
        info: Info {
            key_attribute: "stack_id",
            service_name: "cloudformation",
            resource_type_name: "stack",
            list_api: ListApi::Xml(XmlListApi {
                method: XmlListMethod::Post,
                service_name: "cloudformation",
                action: Some("DescribeStacks"),
                version: Some("2010-05-15"),
                iteration_tag: vec!["member"],
                limit: None,
                params: vec![],
            }),
            get_api: None,
            list_api_document_url:
                "https://docs.aws.amazon.com/AWSCloudFormation/latest/APIReference/API_DescribeStacks.html",
            get_api_document_url: None,
            resource_url: Some( "cloudformation/home?#/stacks/stackinfo?stackId={stack_id}"),
        },
    }
}

impl AwsResource for Resource {
    fn info(&self) -> &Info {
        &self.info
    }

    fn matching_sub_command(&self) -> Option<SubCommand> {
        Some(SubCommand::Cloudformation {
            command: CloudformationCommand::Stack,
        })
    }

    fn make_vec(&self, yaml: &Yaml) -> (ResourceList, Option<String>) {
        make_vec(self, &yaml["describe_stacks_result"]["stacks"])
    }

    fn header(&self) -> Vec<&'static str> {
        vec!["name", "status", "creation time"]
    }

    fn line(&self, list: &Yaml, _get: &Option<Yaml>) -> Vec<String> {
        vec![
            show::raw(&list["stack_name"]),
            show::raw(&list["stack_status"]),
            show::time(&list["creation_time"]),
        ]
    }

    fn detail(&self, list: &Yaml, get: &Option<Yaml>, region: &str) -> Section {
        Section::new(&list)
            .yaml_name("stack_name")
            .resource_url(self.console_url(list, get, region))
            .raw1("status", "stack_status")
            .raw("description")
            .raw("creation_time")
            .raw("last_updated_time")
    }

    fn url_params(&self, list: &Yaml, _get: &Option<Yaml>) -> Option<Vec<(&'static str, String)>> {
        Some(vec![("stack_id", show::raw(&list["stack_id"]))])
    }
}