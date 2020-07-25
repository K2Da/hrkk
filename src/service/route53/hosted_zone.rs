use crate::service::prelude::*;

#[derive(Serialize)]
pub(crate) struct Resource {
    info: Info,
}

pub(crate) fn new() -> Resource {
    Resource {
        info: Info {
            key_attribute: Some("id"),
            service_name: "route53",
            resource_type_name: "hosted_zone",
            list_api: ListApi {
                format: ListFormat::Xml(ListXml {
                    path: "/2013-04-01/hostedzone",
                    path_place_holder: None,
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
            resource_url: Some(ResourceUrl::Global(
                "route53/home?#resource-record-sets:{zone_id}",
            )),
        },
    }
}

impl AwsResource for Resource {
    fn info(&self) -> &Info {
        &self.info
    }

    fn matching_sub_command(&self) -> Option<SubCommand> {
        Some(SubCommand::Route53 {
            command: Route53Command::HostedZone,
        })
    }

    fn make_vec(&self, yaml: &Yaml) -> (ResourceList, Option<String>) {
        make_vec(self, &yaml["hosted_zones"], Some("marker"))
    }

    fn header(&self) -> Vec<&'static str> {
        vec!["id", "name"]
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
