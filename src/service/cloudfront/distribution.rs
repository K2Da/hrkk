use crate::service::prelude::*;

#[derive(Serialize)]
pub(crate) struct Resource {
    info: Info,
}

pub(crate) fn new() -> Resource {
    Resource {
        info: Info {
            key_attribute: "id",
            service_name: "cloudfront",
            resource_type_name: "distribution",
            list_api: ListApi::Xml(XmlListApi {
                path: "/2020-05-31/distribution",
                path_place_holder: None,
                method: XmlListMethod::Get,
                service_name: "cloudfront",
                iteration_tag: vec!["DistributionSummary", "SslProtocol", "Method", "Name", "Origin"],
                limit: Some(Limit {
                    name: "maxitems",
                    max: 100,
                }),
                token_name: "marker",
                params: vec![],
                region: Some(Region::UsEast1),
            }),
            get_api: None,

            list_api_document_url:
                "https://docs.aws.amazon.com/cloudfront/latest/APIReference/API_ListDistributions.html",
            get_api_document_url: None,
            resource_url: Some(ResourceUrl::Global("cloudfront/home?#distribution-settings:{distribution_id}")),
        },
    }
}

impl AwsResource for Resource {
    fn info(&self) -> &Info {
        &self.info
    }

    fn matching_sub_command(&self) -> Option<SubCommand> {
        Some(SubCommand::Cloudfront {
            command: CloudfrontCommand::Distribution,
        })
    }

    fn make_vec(&self, yaml: &Yaml) -> (ResourceList, Option<String>) {
        make_vec(self, &yaml["items"], "marker")
    }

    fn header(&self) -> Vec<&'static str> {
        vec!["id", "comment"]
    }

    fn line(&self, list: &Yaml, _get: &Option<Yaml>) -> Vec<String> {
        vec![show::raw(&list["id"]), show::raw(&list["comment"])]
    }

    fn detail(&self, list: &Yaml, get: &Option<Yaml>, region: &str) -> Section {
        Section::new(&list)
            .yaml_name("id")
            .resource_url(self.console_url(list, get, region))
            .raw("status")
            .raw("enabled")
            .raw("comment")
            .raw("arn")
            .raw("price_class")
    }

    fn url_params(&self, list: &Yaml, _get: &Option<Yaml>) -> Option<Vec<(&'static str, String)>> {
        Some(vec![("distribution_id", show::raw(&list["id"]))])
    }
}