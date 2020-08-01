use crate::service::prelude::*;

#[derive(Serialize)]
pub(crate) struct Resource {
    info: Info,
}

pub(crate) fn new() -> Resource {
    Resource {
        info: Info {
            sub_command: Some(SubCommand::Route53 { command: Route53::HostedZone }),
            key_attribute: Some("id"),
            service_name: "route53",
            resource_type_name: "hosted_zone",
            header: vec!["id", "name"],
            list_api: ListApi {
                format: ListFormat::Xml(ListXml {
                    path: ("/2013-04-01/hostedzone", None),
                    method: Method::Get,
                    service_name: "route53",
                    iteration_tag: vec!["HostedZone"],
                    limit: Some(Limit {
                        name: "maxitems",
                        max: 100,
                    }),
                    token_name: "marker",
                    params: vec![],
                    region: Some(Region::UsEast1),
                }),
                document: "https://docs.aws.amazon.com/Route53/latest/APIReference/API_ListHostedZones.html",
            },
            get_api: None,
            resource_url: Some(Global("route53/home?#resource-record-sets:{zone_id}")),
        },
    }
}

impl AwsResource for Resource {
    fn info(&self) -> &Info {
        &self.info
    }

    fn list_and_next_token(&self, yaml: &Yaml) -> (ResourceList, Option<String>) {
        (
            make_resource_list(self, &yaml["hosted_zones"]),
            next_token(&yaml, Some("marker")),
        )
    }

    fn line(&self, list: &Yaml, _get: &Option<Yaml>) -> Vec<String> {
        vec![show::raw(&list["id"]), show::raw(&list["name"])]
    }

    fn detail(&self, list: &Yaml, get: &Option<Yaml>, region: &str) -> Section {
        Section::new(list)
            .yaml_name("name")
            .resource_url(self.console_url(list, get, region))
            .raw("id")
            .raw("caller_reference")
            .raw("resource_record_set_count")
            .section(
                Section::new(&list["config"])
                    .string_name("config")
                    .raw("comment")
                    .raw("private_zone"),
            )
    }

    fn url_params(&self, list: &Yaml, _get: &Option<Yaml>) -> Option<Vec<(&'static str, String)>> {
        Some(vec![("zone_id", self.resource_name(list))])
    }

    fn resource_name(&self, yaml: &Yaml) -> String {
        show::raw(&yaml["id"]).replace("/hostedzone/", "")
    }
}
