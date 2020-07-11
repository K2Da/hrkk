use crate::service::prelude::*;

#[derive(Serialize)]
pub(crate) struct Resource {
    info: Info,
}

pub(crate) fn new() -> Resource {
    Resource {
        info: Info {
            key_attribute: "alarm_arn",
            service_name: "cloudwatch",
            resource_type_name: "alarm",
            api_type: ApiType::Xml(XmlApi {
                service_name: "monitoring",
                action: "DescribeAlarms",
                version: "2010-08-01",
                limit_name: "MaxRecords",
                iteration_tag: vec!["member"],
                max_limit: 100,
            }),
            document_url:
                "https://docs.aws.amazon.com/AmazonCloudWatch/latest/APIReference/API_DescribeAlarms.html",
        },
    }
}

impl AwsResource for Resource {
    fn info(&self) -> &Info {
        &self.info
    }

    fn matching_sub_command(&self) -> Option<SubCommand> {
        Some(SubCommand::Cloudwatch {
            command: CloudwatchCommand::Alarm,
        })
    }

    fn make_vec(&self, yaml: &Yaml) -> (Vec<Yaml>, Option<String>) {
        let mut arr = vec![];
        let yaml = &yaml["describe_alarms_result"];

        if let Yaml::Array(groups) = &yaml["metric_alarms"] {
            arr.append(&mut groups.clone());
        }

        if let Yaml::Array(groups) = &yaml["composite_alarms"] {
            arr.append(&mut groups.clone());
        }

        (arr, next_token(&yaml))
    }

    fn header(&self) -> Vec<&'static str> {
        vec!["state", "name"]
    }

    fn line(&self, item: &Yaml) -> Vec<String> {
        vec![
            show::raw(&item["state_value"]),
            show::raw(&item["alarm_name"]),
        ]
    }

    fn detail(&self, yaml: &Yaml) -> crate::show::Section {
        crate::show::Section::new(&yaml)
            .yaml_name("alarm_name")
            .raw("namespace", "namespace")
            .raw("description", "alarm_description")
            .section(
                crate::show::Section::new(&yaml)
                    .string_name("state")
                    .raw("value", "state_value")
                    .raw("reason", "state_reason")
                    .raw("reason data", "state_reason_data")
                    .raw("updated", "state_updated_timestamp"),
            )
    }
}
