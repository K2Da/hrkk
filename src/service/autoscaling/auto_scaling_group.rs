use crate::service::prelude::*;

#[derive(Serialize)]
pub(crate) struct Resource {
    info: Info,
}

pub(crate) fn new() -> Resource {
    Resource {
        info: Info {
            key_attribute: "auto_scaling_group_name",
            service_name: "autoscaling",
            resource_type_name: "auto_scaling_group",
            list_api: ListApi::Xml(XmlListApi {
                method: XmlListMethod::Post,
                service_name: "autoscaling",
                action: Some("DescribeAutoScalingGroups"),
                version: Some("2011-01-01"),
                iteration_tag: vec!["member"],
                limit: Some(Limit {
                    name: "MaxResults",
                    max: 100,
                }),
                params: vec![],
            }),
            list_api_document_url:
                "https://docs.aws.amazon.com/autoscaling/ec2/APIReference/API_DescribeAutoScalingGroups.html",
            get_api: None,
            get_api_document_url: None,
            resource_url: Some("ec2autoscaling/home?#/details/{group_name}"),
        },
    }
}

impl AwsResource for Resource {
    fn info(&self) -> &Info {
        &self.info
    }

    fn matching_sub_command(&self) -> Option<SubCommand> {
        Some(SubCommand::Autoscaling {
            command: AutoscalingCommand::AutoScalingGroup,
        })
    }

    fn make_vec(&self, yaml: &Yaml) -> (ResourceList, Option<String>) {
        make_vec(
            self,
            &yaml["describe_auto_scaling_groups_result"]["auto_scaling_groups"],
        )
    }

    fn header(&self) -> Vec<&'static str> {
        vec!["name", "created"]
    }

    fn line(&self, list: &Yaml, _get: &Option<Yaml>) -> Vec<String> {
        vec![
            show::raw(&list["auto_scaling_group_name"]),
            show::raw(&list["created_time"]),
        ]
    }

    fn detail(&self, list: &Yaml, get: &Option<Yaml>, region: &str) -> Section {
        Section::new(&list)
            .yaml_name("auto_scaling_group_name")
            .resource_url(self.console_url(list, get, region))
            .raw("desired_capacity")
            .raw("min_size")
            .raw("max_size")
            .time("created_time")
    }

    fn url_params(&self, list: &Yaml, _get: &Option<Yaml>) -> Option<Vec<(&'static str, String)>> {
        Some(vec![(
            "group_name",
            show::raw(&list["auto_scaling_group_name"]),
        )])
    }
}