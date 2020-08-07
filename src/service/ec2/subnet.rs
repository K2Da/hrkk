use crate::service::prelude::*;

#[derive(Serialize)]
pub(crate) struct Resource {
    info: Info,
}

pub(crate) fn new() -> Resource {
    Resource {
        info: Info {
            sub_command: Some(SubCommand::Ec2 {
                command: Ec2::Subnet,
            }),
            key_attribute: Some("subnet_id"),
            service_name: "ec2",
            resource_type_name: "subnet",
            header: vec!["id", "name"],
            list_api: ListApi {
                format: ListFormat::Xml(ListXml {
                    path: ("/", None),
                    method: Method::Post,
                    service_name: "ec2",
                    iteration_tag: vec!["item"],
                    limit: None,
                    token_name: "NextToken",
                    params: vec![("Action", "DescribeSubnets"), ("Version", "2016-11-15")],
                    region: None,
                }),
                document: DocumentUrl("AWSEC2/latest/APIReference/API_DescribeSubnets.html"),
            },
            get_api: None,
            resource_url: Some(Regional("vpc/home?#subnets:search={subnet_id}")),
        },
    }
}

impl AwsResource for Resource {
    fn info(&self) -> &Info {
        &self.info
    }

    fn list_and_next_token(&self, yaml: &Yaml) -> (ResourceList, Option<String>) {
        (make_resource_list(self, &yaml["subnet_set"]), None)
    }

    fn line(&self, list: &Yaml, _get: &Option<Yaml>) -> Vec<String> {
        vec![
            raw(&list["subnet_id"]),
            raw(&tag_value(&list["tag_set"], "Name")),
        ]
    }

    fn detail(&self, list: &Yaml, get: &Option<Yaml>, region: &str) -> Section {
        Section::new(list)
            .tag_name("tag_set", "Name")
            .resource_url(self.console_url(list, get, region))
            .raw("subnet_id")
            .raw("subnet_arn")
            .raw("state")
            .raw("owner_id")
            .raw("vpc_id")
            .raw("cidr_block")
            .raw("ipv_6_cidr_block_association_set")
            .raw("available_ip_address_count")
            .raw("availability_zone")
            .raw("availability_zone_id")
            .raw("default_for_az")
            .raw("map_public_ip_on_launch")
            .raw("assign_ipv_6_address_on_creation")
            .section(
                Section::new(list)
                    .string_name("tags")
                    .yaml_pairs("tag_set", ("key", "value")),
            )
    }

    fn url_params(&self, list: &Yaml, _get: &Option<Yaml>) -> Option<Vec<(&'static str, String)>> {
        Some(vec![("subnet_id", raw(&list["subnet_id"]))])
    }
}
