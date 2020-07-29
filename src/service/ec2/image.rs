use crate::service::prelude::*;

#[derive(Serialize)]
pub(crate) struct Resource {
    info: Info,
}

pub(crate) fn new() -> Resource {
    Resource {
        info: Info {
            key_attribute: Some("image_id"),
            service_name: "ec2",
            resource_type_name: "image",
            list_api: ListApi {
                format: ListFormat::Xml(ListXml {
                    path: ("/", None),
                    method: Method::Post,
                    service_name: "ec2",
                    iteration_tag: vec!["item"],
                    limit: None,
                    token_name: "NextToken",
                    params: vec![
                        ("Owner.1", "self"),
                        ("Action", "DescribeImages"),
                        ("Version", "2016-11-15"),
                    ],
                    region: None,
                }),
                document:
                    "https://docs.aws.amazon.com/AWSEC2/latest/APIReference/API_DescribeImages.html",
            },
            get_api: None,
            resource_url: Some(Regional("ec2/v2/home?#Images:imageId={image_id}")),
        },
    }
}

impl AwsResource for Resource {
    fn info(&self) -> &Info {
        &self.info
    }

    fn matching_sub_command(&self) -> Option<SubCommand> {
        Some(SubCommand::Ec2 {
            command: Ec2Command::Image,
        })
    }

    fn list_and_next_token(&self, yaml: &Yaml) -> (ResourceList, Option<String>) {
        (make_resource_list(self, &yaml["images_set"]), None)
    }

    fn header(&self) -> Vec<&'static str> {
        vec!["name", "description"]
    }

    fn line(&self, list: &Yaml, _get: &Option<Yaml>) -> Vec<String> {
        vec![show::raw(&list["name"]), show::raw(&list["description"])]
    }

    fn detail(&self, list: &Yaml, get: &Option<Yaml>, region: &str) -> Section {
        Section::new(list)
            .yaml_name("name")
            .resource_url(self.console_url(list, get, region))
            .raw("description")
            .raw("is_public")
            .time("creation_date")
            .raw("architecture_type")
            .raw("image_type")
            .raw("virtualization_type")
            .raw("hypervisor")
            .raw("platform_details")
            .raw("usage_operation")
    }

    fn url_params(&self, list: &Yaml, _get: &Option<Yaml>) -> Option<Vec<(&'static str, String)>> {
        Some(vec![("image_id", show::raw(&list["image_id"]))])
    }
}
