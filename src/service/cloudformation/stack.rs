use crate::service::prelude::*;

#[derive(Serialize)]
pub(crate) struct Resource {
    info: Info,
}

pub(crate) fn new() -> Resource {
    Resource {
        info: Info {
            sub_command: Some(SubCommand::Cloudformation { command: Cloudformation::Stack }),
            key_attribute: Some("stack_id"),
            service_name: "cloudformation",
            resource_type_name: "stack",
            header: vec!["name", "status", "creation time"],
            list_api: ListApi {
                format: ListFormat::Xml(ListXml {
                    path: ("/", None),
                    method: Method::Post,
                    service_name: "cloudformation",
                    iteration_tag: vec!["member"],
                    limit: None,
                    token_name: "NextToken",
                    params: vec![
                        ("Action", "DescribeStacks"),
                        ("Version", "2010-05-15")
                    ],
                    region: None,
                }),
                document: "https://docs.aws.amazon.com/AWSCloudFormation/latest/APIReference/API_DescribeStacks.html",
            },
            get_api: None,
            resource_url: Some(Regional("cloudformation/home?#/stacks/stackinfo?stackId={stack_id}")),
        },
    }
}

impl AwsResource for Resource {
    fn info(&self) -> &Info {
        &self.info
    }

    fn list_and_next_token(&self, yaml: &Yaml) -> (ResourceList, Option<String>) {
        (
            make_resource_list(self, &yaml["describe_stacks_result"]["stacks"]),
            next_token(&yaml["describe_stacks_result"], Some("next_token")),
        )
    }

    fn line(&self, list: &Yaml, _get: &Option<Yaml>) -> Vec<String> {
        vec![
            show::raw(&list["stack_name"]),
            show::raw(&list["stack_status"]),
            show::time(&list["creation_time"]),
        ]
    }

    fn detail(&self, list: &Yaml, get: &Option<Yaml>, region: &str) -> Section {
        Section::new(list)
            .yaml_name("stack_name")
            .resource_url(self.console_url(list, get, region))
            .raw_n("status", &["stack_status"])
            .raw("description")
            .raw("creation_time")
            .raw("last_updated_time")
    }

    fn url_params(&self, list: &Yaml, _get: &Option<Yaml>) -> Option<Vec<(&'static str, String)>> {
        Some(vec![("stack_id", show::raw(&list["stack_id"]))])
    }
}
