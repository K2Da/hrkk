use crate::service::prelude::*;

#[derive(Serialize)]
pub(crate) struct Resource {
    info: Info,
}

pub(crate) fn new() -> Resource {
    Resource {
        info: Info {
            key_attribute: Some("auto_scaling_group_name"),
            service_name: "autoscaling",
            resource_type_name: "auto_scaling_group",
            header: vec!["name", "created"],
            list_api: ListApi {
                format: ListFormat::Xml(ListXml {
                    path: ("/", None),
                    method: Method::Post,
                    service_name: "autoscaling",
                    iteration_tag: vec!["member"],
                    limit: Some(Limit {
                        name: "MaxResults",
                        max: 100,
                    }),
                    token_name: "NextToken",
                    params: vec![
                        ("Action", "DescribeAutoScalingGroups"),
                        ("Version", "2011-01-01")
                    ],
                    region: None,
                    }),
                document: "https://docs.aws.amazon.com/autoscaling/ec2/APIReference/API_DescribeAutoScalingGroups.html",
            },
            get_api: None,
            resource_url: Some(Regional("ec2autoscaling/home?#/details/{group_name}")),
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

    fn list_and_next_token(&self, yaml: &Yaml) -> (ResourceList, Option<String>) {
        (
            make_resource_list(
                self,
                &yaml["describe_auto_scaling_groups_result"]["auto_scaling_groups"],
            ),
            next_token(
                &yaml["describe_auto_scaling_groups_result"],
                Some("next_token"),
            ),
        )
    }

    fn line(&self, list: &Yaml, _get: &Option<Yaml>) -> Vec<String> {
        vec![
            show::raw(&list["auto_scaling_group_name"]),
            show::raw(&list["created_time"]),
        ]
    }

    fn detail(&self, list: &Yaml, get: &Option<Yaml>, region: &str) -> Section {
        Section::new(list)
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
