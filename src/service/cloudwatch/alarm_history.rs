use crate::service::prelude::*;

#[derive(Serialize)]
pub(crate) struct Resource {
    info: Info,
}

pub(crate) fn new() -> Resource {
    Resource {
        info: Info {
            key_attribute: "alarm_name",
            service_name: "cloudwatch",
            resource_type_name: "alarm_history",
            api_type: ApiType::Xml(XmlApi {
                service_name: "monitoring",
                action: "DescribeAlarmHistory",
                version: "2010-08-01",
                limit_name: "MaxRecords",
                iteration_tag: vec!["member"],
                max_limit: 100,
            }),
            document_url:
                "https://docs.aws.amazon.com/AmazonCloudWatch/latest/APIReference/API_DescribeAlarmHistory.html",
        },
    }
}

impl AwsResource for Resource {
    fn info(&self) -> &Info {
        &self.info
    }

    fn matching_sub_command(&self) -> Option<SubCommand> {
        Some(SubCommand::Cloudwatch {
            command: CloudwatchCommand::AlarmHistory,
        })
    }

    fn make_vec(&self, yaml: &Yaml) -> (Vec<Yaml>, Option<String>) {
        let mut arr = vec![];
        let yaml = &yaml["describe_alarm_history_result"];

        if let Yaml::Array(items) = &yaml["alarm_history_items"] {
            arr.append(&mut items.clone());
        }

        (arr, next_token(&yaml))
    }

    fn header(&self) -> Vec<&'static str> {
        vec!["time", "name", "summary"]
    }

    fn line(&self, item: &Yaml) -> Vec<String> {
        vec![
            show::time(&item["timestamp"]),
            show::raw(&item["alarm_name"]),
            show::raw(&item["history_summary"]),
        ]
    }

    fn detail(&self, yaml: &Yaml) -> crate::show::Section {
        crate::show::Section::new(&yaml)
            .yaml_name("alarm_name")
            .raw("alarm type", "alarm_type")
            .raw("item type", "history_item_type")
            .raw("summary", "history_summary")
            .time("timestamp", "timestamp")
            .raw("data", "history_data")
    }
}
