use crate::service::prelude::*;

#[derive(Serialize)]
pub(crate) struct Resource {
    info: Info,
}

pub(crate) fn new() -> Resource {
    Resource {
        info: Info {
            sub_command: Some(SubCommand::Ec2 {
                command: Ec2::SecurityGroup,
            }),
            key_attribute: Some("group_id"),
            service_name: "ec2",
            resource_type_name: "security_group",
            header: vec!["id", "name"],
            list_api: ListApi {
                format: ListFormat::Xml(ListXml {
                    path: ("/", None),
                    method: Method::Get,
                    service_name: "ec2",
                    iteration_tag: vec!["item"],
                    limit: None,
                    token_name: "NextToken",
                    params: vec![
                        ("Action", "DescribeSecurityGroups"),
                        ("Version", "2016-11-15"),
                    ],
                    region: None,
                }),
                document: DocumentUrl("AWSEC2/latest/APIReference/API_DescribeSecurityGroups.html"),
            },
            get_api: None,
            resource_url: Some(Regional("vpc/home?#SecurityGroup:groupId={group_id}")),
        },
    }
}

impl AwsResource for Resource {
    fn info(&self) -> &Info {
        &self.info
    }

    fn list_and_next_token(&self, yaml: &Yaml) -> (ResourceList, Option<String>) {
        (make_resource_list(self, &yaml["security_group_info"]), None)
    }

    fn line(&self, list: &Yaml, _get: &Option<Yaml>) -> Vec<String> {
        vec![raw(&list["group_id"]), raw(&list["group_name"])]
    }

    fn detail(&self, list: &Yaml, get: &Option<Yaml>, region: &str) -> Section {
        Section::new(list)
            .yaml_name("group_name")
            .resource_url(self.console_url(list, get, region))
            .raw("group_id")
            .raw("owner_id")
            .raw("group_description")
            .raw("vpc_id")
            .raw("ip_permissions")
            .section(
                Section::new(list)
                    .string_name("tags")
                    .yaml_pairs("tag_set", ("key", "value")),
            )
    }

    fn url_params(&self, list: &Yaml, _get: &Option<Yaml>) -> Option<Vec<ParamSet>> {
        Some(vec![("group_id", raw(&list["group_id"]), true)])
    }
}
