use crate::service::prelude::*;

#[derive(Serialize)]
pub(crate) struct Resource {
    info: Info,
}

pub(crate) fn new() -> Resource {
    Resource {
        info: Info {
            sub_command: Some(SubCommand::Cloudfront {
                command: Cloudfront::Distribution,
            }),
            key_attribute: Some("id"),
            service_name: "cloudfront",
            resource_type_name: "distribution",
            header: vec!["id", "comment"],
            list_api: ListApi {
                format: ListFormat::Xml(ListXml {
                    path: ("/2020-05-31/distribution", None),
                    method: Method::Get,
                    service_name: "cloudfront",
                    iteration_tag: vec![
                        "DistributionSummary",
                        "SslProtocol",
                        "Method",
                        "Name",
                        "Origin",
                    ],
                    limit: Some(Limit {
                        name: "maxitems",
                        max: 100,
                    }),
                    token_name: "marker",
                    params: vec![],
                    region: Some(Region::UsEast1),
                }),
                document: DocumentUrl("cloudfront/latest/APIReference/API_ListDistributions.html"),
            },
            get_api: None,
            resource_url: Some(Global(
                "cloudfront/home?#distribution-settings:{distribution_id}",
            )),
        },
    }
}

impl AwsResource for Resource {
    fn info(&self) -> &Info {
        &self.info
    }

    fn list_and_next_token(&self, yaml: &Yaml) -> (ResourceList, Option<String>) {
        (
            make_resource_list(self, &yaml["items"]),
            next_token(&yaml, Some("marker")),
        )
    }

    fn line(&self, list: &Yaml, _get: &Option<Yaml>) -> Vec<String> {
        vec![raw(&list["id"]), raw(&list["comment"])]
    }

    fn detail(&self, list: &Yaml, get: &Option<Yaml>, region: &str) -> Section {
        Section::new(list)
            .yaml_name("id")
            .resource_url(self.console_url(list, get, region))
            .raw("status")
            .raw("enabled")
            .raw("comment")
            .raw("arn")
            .raw("price_class")
    }

    fn url_params(&self, list: &Yaml, _get: &Option<Yaml>) -> Option<Vec<ParamSet>> {
        Some(vec![("distribution_id", raw(&list["id"]), true)])
    }
}
