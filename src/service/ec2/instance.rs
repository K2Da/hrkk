use crate::service::prelude::*;

#[derive(Serialize)]
pub(crate) struct Resource {
    info: Info,
}

pub(crate) fn new() -> Resource {
    Resource {
        info: Info {
            key_attribute: Some("instance_id"),
            service_name: "ec2",
            resource_type_name: "instance",
            list_api: ListApi {
                format: ListFormat::Xml(ListXml {
                    path: ("/", None),
                    method: Method::Post,
                    service_name: "ec2",
                    iteration_tag: vec!["item"],
                    limit: Some(Limit {
                        name: "MaxResults",
                        max: 1000,
                    }),
                    token_name: "NextToken",
                    params: vec![("Action", "DescribeInstances"), ("Version", "2016-11-15")],
                    region: None,
                }),
                document: "https://docs.aws.amazon.com/AWSEC2/latest/APIReference/API_DescribeInstances.html",
            },
            get_api: None,
            resource_url: Some(Regional(
                "ec2/v2/home?#Instances:search={instance_id}",
            )),
        },
    }
}

impl AwsResource for Resource {
    fn info(&self) -> &Info {
        &self.info
    }

    fn matching_sub_command(&self) -> Option<SubCommand> {
        Some(SubCommand::Ec2 {
            command: Ec2Command::Instance,
        })
    }

    fn list_and_next_token(&self, yaml: &Yaml) -> (ResourceList, Option<String>) {
        let mut result = vec![];

        if let Yaml::Array(reservations) = &yaml["reservation_set"] {
            for reservation in reservations {
                if let Yaml::Array(instances) = &reservation["instances_set"] {
                    for instance in instances {
                        result.push((self.line(instance, &None), instance.clone()));
                    }
                }
            }
        }

        (result, next_token(&yaml, Some("next_token")))
    }

    fn header(&self) -> Vec<&'static str> {
        vec!["instance id", "state", "name"]
    }

    fn line(&self, list: &Yaml, _get: &Option<Yaml>) -> Vec<String> {
        vec![
            show::raw(&list["instance_id"]),
            show::raw(&list["instance_state"]["name"]),
            show::raw(&tag_value(&list["tag_set"], "Name")),
        ]
    }

    fn detail(&self, list: &Yaml, get: &Option<Yaml>, region: &str) -> Section {
        Section::new(list)
            .tag_name("tag_set", "Name")
            .resource_url(self.console_url(list, get, region))
            .raw("instance_id")
            .raw("instance_type")
            .raw("architecture")
            .raw_n("state", &["instance_state", "name"])
            .section(
                Section::new(list)
                    .string_name("tags")
                    .yaml_pairs("tag_set", ("key", "value")),
            )
            .section(
                Section::new(list)
                    .string_name("network")
                    .raw("subnet_id")
                    .raw("private_ip_address")
                    .raw_n("availability zone", &["placement", "availability_zone"]),
            )
            .section(
                Section::new(list)
                    .string_name("device")
                    .raw("root_device_type")
                    .raw("root_device_name"),
            )
    }

    fn url_params(&self, list: &Yaml, _get: &Option<Yaml>) -> Option<Vec<(&'static str, String)>> {
        Some(vec![("instance_id", show::raw(&list["instance_id"]))])
    }
}
