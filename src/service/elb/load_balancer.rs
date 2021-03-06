use crate::service::prelude::*;

#[derive(Serialize)]
pub(crate) struct Resource {
    info: Info,
}

pub(crate) fn new() -> Resource {
    Resource {
        info: Info {
            sub_command: Some(SubCommand::Elb {
                command: Elb::LoadBalancer,
            }),
            key_attribute: Some("load_balancer_name"),
            service_name: "elb",
            resource_type_name: "load_balancer",
            header: vec!["name", "type"],
            list_api: ListApi {
                format: ListFormat::Xml(ListXml {
                    path: ("/", None),
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
                document: DocumentUrl(
                    "elasticloadbalancing/latest/APIReference/API_DescribeLoadBalancers.html",
                ),
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

    fn list_and_next_token(&self, yaml: &Yaml) -> (ResourceList, Option<String>) {
        (
            make_resource_list(
                self,
                &yaml["describe_load_balancers_result"]["load_balancers"],
            ),
            next_token(&yaml["describe_load_balancers_result"], Some("marker")),
        )
    }

    fn line(&self, list: &Yaml, _get: &Option<Yaml>) -> Vec<String> {
        vec![raw(&list["load_balancer_name"]), raw(&list["type"])]
    }

    fn detail(&self, list: &Yaml, get: &Option<Yaml>, region: &str) -> Section {
        Section::new(list)
            .yaml_name("load_balancer_name")
            .resource_url(self.console_url(list, get, region))
            .raw("load_balancer_arn")
            .raw("type")
            .raw("scheme")
            .raw_n("state", &["state", "code"])
            .raw("canonical_hosted_zone_id")
    }

    fn url_params(&self, list: &Yaml, _get: &Option<Yaml>) -> Option<Vec<ParamSet>> {
        Some(vec![("dns_name", raw(&list["dns_name"]), true)])
    }
}
