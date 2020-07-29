use crate::service::prelude::*;

#[derive(Serialize)]
pub(crate) struct Resource {
    info: Info,
}

pub(crate) fn new() -> Resource {
    Resource {
        info: Info {
            key_attribute: Some("launch_template_id"),
            service_name: "ec2",
            resource_type_name: "launch_template",
            list_api: ListApi {
                format: ListFormat::Xml(ListXml {
                    path: ("/", None),
                    method: Method::Post,
                    service_name: "ec2",
                    iteration_tag: vec!["item"],
                    limit: Some(Limit {
                        name: "MaxResults",
                        max: 200,
                    }),
                    token_name: "NextToken",
                    params: vec![
                        ("Action", "DescribeLaunchTemplates"),
                        ("Version", "2016-11-15"),
                    ],
                    region: None,
                }),
                document: "https://docs.aws.amazon.com/AWSEC2/latest/APIReference/API_DescribeLaunchTemplates.html",
            },
            get_api: None,
            resource_url: Some(Regional("ec2/v2/home?#LaunchTemplateDetails:launchTemplateId={template_id}")),
        },
    }
}

impl AwsResource for Resource {
    fn info(&self) -> &Info {
        &self.info
    }

    fn matching_sub_command(&self) -> Option<SubCommand> {
        Some(SubCommand::Ec2 {
            command: Ec2Command::LaunchTemplate,
        })
    }

    fn list_and_next_token(&self, yaml: &Yaml) -> (ResourceList, Option<String>) {
        (
            make_resource_list(self, &yaml["launch_templates"]),
            next_token(&yaml, Some("next_token")),
        )
    }

    fn header(&self) -> Vec<&'static str> {
        vec!["name", "latest version", "default version"]
    }

    fn line(&self, list: &Yaml, _get: &Option<Yaml>) -> Vec<String> {
        vec![
            show::raw(&list["launch_template_name"]),
            show::raw(&list["latest_version_number"]),
            show::raw(&list["default_version_number"]),
        ]
    }

    fn detail(&self, list: &Yaml, get: &Option<Yaml>, region: &str) -> Section {
        Section::new(list)
            .yaml_name("launch_template_name")
            .resource_url(self.console_url(list, get, region))
            .raw("template_id")
            .raw("latest_version_number")
            .raw("default_version_number")
            .time("create_time")
            .raw("created_by")
    }

    fn url_params(&self, list: &Yaml, _get: &Option<Yaml>) -> Option<Vec<(&'static str, String)>> {
        Some(vec![(
            "template_id",
            show::raw(&list["launch_template_id"]),
        )])
    }
}
