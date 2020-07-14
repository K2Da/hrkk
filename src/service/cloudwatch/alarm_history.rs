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
            list_api: ListApi::Xml(XmlListApi {
                method: XmlListMethod::Post,
                service_name: "monitoring",
                action: Some("DescribeAlarmHistory"),
                version: Some("2010-08-01"),
                iteration_tag: vec!["member"],
                limit: Some(Limit {
                    name: "MaxRecords",
                    max: 100,
                }),
                params: vec![],
            }),
            get_api: None,
            list_api_document_url:
                "https://docs.aws.amazon.com/AmazonCloudWatch/latest/APIReference/API_DescribeAlarmHistory.html",
            get_api_document_url: None,
            resource_url: Some(
                "cloudwatch/home?#alarmsV2:alarm/{alarm_name}",
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
            command: CloudwatchCommand::AlarmHistory,
        })
    }

    fn make_vec(&self, yaml: &Yaml) -> (ResourceList, Option<String>) {
        make_vec(
            self,
            &yaml["describe_alarm_history_result"]["alarm_history_items"],
        )
    }

    fn header(&self) -> Vec<&'static str> {
        vec!["time", "name", "summary"]
    }

    fn line(&self, list: &Yaml, _get: &Option<Yaml>) -> Vec<String> {
        vec![
            show::time(&list["timestamp"]),
            show::raw(&list["alarm_name"]),
            show::raw(&list["history_summary"]),
        ]
    }

    fn detail(&self, list: &Yaml, get: &Option<Yaml>, region: &str) -> Section {
        Section::new(&list)
            .yaml_name("alarm_name")
            .resource_url(self.console_url(list, get, region))
            .raw("alarm_type")
            .raw("history_item_type")
            .raw("history_summary")
            .time("timestamp")
            .raw("history_data")
    }

    fn url_params(&self, list: &Yaml, _get: &Option<Yaml>) -> Option<Vec<(&'static str, String)>> {
        Some(vec![("alarm_name", show::raw(&list["alarm_name"]))])
    }
}
