use crate::service::prelude::*;

#[derive(Serialize)]
pub(crate) struct Resource {
    info: Info,
}

pub(crate) fn new() -> Resource {
    Resource {
        info: Info {
            key_attribute: Some("load_balancer_name"),
            service_name: "elb",
            resource_type_name: "load_balancer",
            list_api: ListApi {
                format: ListFormat::Xml(ListXml {
                    path: "/",
                    path_place_holder: None,
                    method: Method::Get,
                    service_name: "elasticloadbalancing",
                    iteration_tag: vec!["member"],
                    limit: Some(Limit {
                        name: "PageSize",
                        max: 400,
                    }),
                    token_name: "Marker",
                    params: vec![
                        ("Action", "DescribeLoadBalancers"),
                        ("Version", "2015-12-01"),
                    ],
                    region: None,
                }),
                document: "https://docs.aws.amazon.com/elasticloadbalancing/latest/APIReference/API_DescribeLoadBalancers.html",
            },
            get_api: None,
            resource_url: Some(Regional("ec2/v2/home?#LoadBalancers:search={dns_name}")),
        },
    }
}

impl AwsResource for Resource {
    fn info(&self) -> &Info {
        &self.info
    }

    fn matching_sub_command(&self) -> Option<SubCommand> {
        Some(SubCommand::Elb {
            command: ElbCommand::LoadBalancer,
        })
    }

    fn make_vec(&self, yaml: &Yaml) -> (ResourceList, Option<String>) {
        make_vec(
            self,
            &yaml["describe_load_balancers_result"]["load_balancers"],
            Some("marker"),
        )
    }

    fn header(&self) -> Vec<&'static str> {
        vec!["name", "type"]
    }

    fn line(&self, list: &Yaml, _get: &Option<Yaml>) -> Vec<String> {
        vec![
            show::raw(&list["load_balancer_name"]),
            show::raw(&list["type"]),
        ]
    }

    fn detail(&self, list: &Yaml, get: &Option<Yaml>, region: &str) -> Section {
        Section::new(list)
            .yaml_name("load_balancer_name")
            .resource_url(self.console_url(list, get, region))
            .raw("load_balancer_arn")
            .raw("type")
            .raw("scheme")
            .raw2("state", ("state", "code"))
            .raw("canonical_hosted_zone_id")
    }

    fn url_params(&self, list: &Yaml, _get: &Option<Yaml>) -> Option<Vec<(&'static str, String)>> {
        Some(vec![("dns_name", show::raw(&list["dns_name"]))])
    }
}
