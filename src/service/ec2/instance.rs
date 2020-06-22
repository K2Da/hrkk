use crate::service::prelude::*;

#[derive(Serialize)]
pub struct Resource {
    info: Info,
}

pub fn new() -> Resource {
    Resource {
        info: Info {
            key_attribute: "instance_id",
            service_name: "ec2",
            resource_type_name: "instance",
            api_type: ApiType::Xml {
                service_name: "ec2",
                action: "DescribeInstances",
                version: "2016-11-15",
                limit_name: "MaxResults",
                iteration_tag: vec!["item"],
            },

            document_url:
                "https://docs.aws.amazon.com/AWSEC2/latest/APIReference/API_DescribeInstances.html",
            max_limit: 1000,
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

    fn make_vec(&self, yaml: &Yaml) -> (Vec<Yaml>, Option<String>) {
        let mut result = vec![];

        if let Yaml::Array(reservations) = &yaml["reservation_set"] {
            for reservation in reservations {
                if let Yaml::Array(instances) = &reservation["instances_set"] {
                    for instance in instances {
                        result.push(instance.clone());
                    }
                }
            }
        }

        (result, next_token(&yaml))
    }

    fn header(&self) -> Vec<&'static str> {
        vec!["instance id", "name"]
    }

    fn header_width(&self) -> Vec<Constraint> {
        vec![Constraint::Length(22), Constraint::Min(0)]
    }

    fn line(&self, item: &Yaml) -> Vec<String> {
        vec![
            show::raw(&item["instance_id"]),
            show::raw(&tag_value(&item["tag_set"], "Name")),
        ]
    }

    fn detail(&self, yaml: &Yaml) -> crate::show::Section {
        crate::show::Section::new(&yaml)
            .tag_name("tag_set", "Name")
            .raw("instance_id", "instance_id")
            .raw("instance_type", "instance_type")
            .raw("architecture", "architecture")
            .raw2("state", ("instance_state", "name"))
            .section(
                crate::show::Section::new(&yaml)
                    .string_name("tags")
                    .yaml_pairs("tag_set", ("key", "value")),
            )
            .section(
                crate::show::Section::new(&yaml)
                    .string_name("network")
                    .raw("subnet id", "subnet_id")
                    .raw("private ip address", "private_ip_address")
                    .raw2("availability zone", ("placement", "availability_zone")),
            )
            .section(
                crate::show::Section::new(&yaml)
                    .string_name("device")
                    .raw("root device type", "root_device_type")
                    .raw("root device name", "root_device_name"),
            )
    }
}
