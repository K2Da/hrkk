use crate::service::prelude::*;

#[derive(Serialize)]
pub(crate) struct Resource {
    info: Info,
}

pub(crate) fn new() -> Resource {
    Resource {
        info: Info {
            sub_command: Some(SubCommand::Ec2 { command: Ec2::Vpc }),
            key_attribute: Some("vpc_id"),
            service_name: "ec2",
            resource_type_name: "vpc",
            header: vec!["name", "state"],
            list_api: ListApi {
                format: ListFormat::Xml(ListXml {
                    path: ("/", None),
                    method: Method::Get,
                    service_name: "ec2",
                    iteration_tag: vec!["item"],
                    limit: Some(Limit {
                        name: "MaxResults",
                        max: 1000,
                    }),
                    token_name: "NextToken",
                    params: vec![("Action", "DescribeVpcs"), ("Version", "2016-11-15")],
                    region: None,
                }),
                document:
                    "https://docs.aws.amazon.com/AWSEC2/latest/APIReference/API_DescribeVpcs.html",
            },
            get_api: None,
            resource_url: Some(Regional("vpc/home?#vpcs:search={vpc_id}")),
        },
    }
}

impl AwsResource for Resource {
    fn info(&self) -> &Info {
        &self.info
    }

    fn list_and_next_token(&self, yaml: &Yaml) -> (ResourceList, Option<String>) {
        (
            make_resource_list(self, &yaml["vpc_set"]),
            next_token(&yaml, Some("next_token")),
        )
    }

    fn line(&self, list: &Yaml, _get: &Option<Yaml>) -> Vec<String> {
        vec![show::raw(&list["vpc_id"]), show::raw(&list["state"])]
    }

    fn detail(&self, list: &Yaml, get: &Option<Yaml>, region: &str) -> Section {
        Section::new(list)
            .yaml_name("vpc_id")
            .resource_url(self.console_url(list, get, region))
            .raw("state")
            .raw("cidr_block")
            .raw("instance_tenancy")
            .raw("is_default")
            .section(
                Section::new(list)
                    .string_name("tags")
                    .yaml_pairs("tag_set", ("key", "value")),
            )
    }

    fn url_params(&self, list: &Yaml, _get: &Option<Yaml>) -> Option<Vec<(&'static str, String)>> {
        Some(vec![("vpc_id", show::raw(&list["vpc_id"]))])
    }
}
