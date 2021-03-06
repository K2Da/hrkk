use crate::service::prelude::*;

#[derive(Serialize)]
pub(crate) struct Resource {
    info: Info,
}

pub(crate) fn new() -> Resource {
    Resource {
        info: Info {
            sub_command: Some(SubCommand::Cloudwatch {
                command: Cloudwatch::Alarm,
            }),
            key_attribute: Some("alarm_arn"),
            service_name: "cloudwatch",
            resource_type_name: "alarm",
            header: vec!["state", "name"],
            list_api: ListApi {
                format: ListFormat::Xml(ListXml {
                    path: ("/", None),
                    method: Method::Post,
                    service_name: "monitoring",
                    iteration_tag: vec!["member"],
                    limit: Some(Limit {
                        name: "MaxRecords",
                        max: 100,
                    }),
                    token_name: "NextToken",
                    params: vec![("Action", "DescribeAlarms"), ("Version", "2010-08-01")],
                    region: None,
                }),
                document: DocumentUrl(
                    "AmazonCloudWatch/latest/APIReference/API_DescribeAlarms.html",
                ),
            },
            get_api: None,
            resource_url: Some(Regional("cloudwatch/home?#alarmsV2:alarm/{alarm_name}")),
        },
    }
}

impl AwsResource for Resource {
    fn info(&self) -> &Info {
        &self.info
    }

    fn list_and_next_token(&self, yaml: &Yaml) -> (ResourceList, Option<String>) {
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

    fn line(&self, list: &Yaml, _get: &Option<Yaml>) -> Vec<String> {
        vec![raw(&list["state_value"]), raw(&list["alarm_name"])]
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
                    .raw_n("value", &["state_value"])
                    .raw_n("reason", &["state_reason"])
                    .raw_n("reason data", &["state_reason_data"])
                    .raw_n("updated", &["state_updated_timestamp"]),
            )
    }

    fn url_params(&self, list: &Yaml, _get: &Option<Yaml>) -> Option<Vec<ParamSet>> {
        Some(vec![("alarm_name", raw(&list["alarm_name"]), true)])
    }
}
