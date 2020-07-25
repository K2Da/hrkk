use crate::service::prelude::*;

#[derive(Serialize)]
pub(crate) struct Resource {
    info: Info,
}

pub(crate) fn new() -> Resource {
    Resource {
        info: Info {
            key_attribute: Some("alarm_arn"),
            service_name: "cloudwatch",
            resource_type_name: "alarm",
            list_api: ListApi {
                format: ListFormat::Xml(ListXml {
                    path: "/",
                    path_place_holder: None,
                    method: Method::Post,
                    service_name: "monitoring",
                    iteration_tag: vec!["member"],
                    limit: Some(Limit {
                        name: "MaxRecords",
                        max: 100,
                    }),
                    token_name: "NextToken",
                    params: vec![
                        ("Action", "DescribeAlarms"),
                        ("Version", "2010-08-01"),
                    ],
                    region: None,
                }),
                document: "https://docs.aws.amazon.com/AmazonCloudWatch/latest/APIReference/API_DescribeAlarms.html",
            },
            get_api: None,
            resource_url: Some(
                Regional("cloudwatch/home?#alarmsV2:alarm/{alarm_name}")
            ),
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

    fn make_vec(&self, yaml: &Yaml) -> (ResourceList, Option<String>) {
        let mut arr = vec![];
        let yaml = &yaml["describe_alarms_result"];

        if let Yaml::Array(groups) = &yaml["metric_alarms"] {
            arr.append(&mut groups.clone());
        }

        if let Yaml::Array(groups) = &yaml["composite_alarms"] {
            arr.append(&mut groups.clone());
        }

        let vec = arr
            .iter()
            .map(|y| (self.line(y, &None), y.clone()))
            .collect();
        (vec, next_token(&yaml, Some("next_token")))
    }

    fn header(&self) -> Vec<&'static str> {
        vec!["state", "name"]
    }

    fn line(&self, list: &Yaml, _get: &Option<Yaml>) -> Vec<String> {
        vec![
            show::raw(&list["state_value"]),
            show::raw(&list["alarm_name"]),
        ]
    }

    fn detail(&self, list: &Yaml, get: &Option<Yaml>, region: &str) -> Section {
        Section::new(list)
            .yaml_name("alarm_name")
            .resource_url(self.console_url(list, get, region))
            .raw("namespace")
            .raw("alarm_description")
            .section(
                Section::new(list)
                    .string_name("state")
                    .raw1("value", "state_value")
                    .raw1("reason", "state_reason")
                    .raw1("reason data", "state_reason_data")
                    .raw1("updated", "state_updated_timestamp"),
            )
    }

    fn url_params(&self, list: &Yaml, _get: &Option<Yaml>) -> Option<Vec<(&'static str, String)>> {
        Some(vec![("alarm_name", show::raw(&list["alarm_name"]))])
    }
}
