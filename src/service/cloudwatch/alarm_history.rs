use crate::service::prelude::*;

#[derive(Serialize)]
pub(crate) struct Resource {
    info: Info,
}

pub(crate) fn new() -> Resource {
    Resource {
        info: Info {
            sub_command: Some(SubCommand::Cloudwatch {
                command: Cloudwatch::AlarmHistory,
            }),
            key_attribute: Some("alarm_name"),
            service_name: "cloudwatch",
            resource_type_name: "alarm_history",
            header: vec!["time", "name", "summary"],
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
                    params: vec![
                        ("Action", "DescribeAlarmHistory"),
                        ("Version", "2010-08-01"),
                    ],
                    region: None,
                }),
                document: DocumentUrl(
                    "AmazonCloudWatch/latest/APIReference/API_DescribeAlarmHistory.html",
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
        (
            make_resource_list(
                self,
                &yaml["describe_alarm_history_result"]["alarm_history_items"],
            ),
            next_token(&yaml["describe_alarm_history_result"], Some("next_token")),
        )
    }

    fn line(&self, list: &Yaml, _get: &Option<Yaml>) -> Vec<String> {
        vec![
            time(&list["timestamp"]),
            raw(&list["alarm_name"]),
            raw(&list["history_summary"]),
        ]
    }

    fn detail(&self, list: &Yaml, get: &Option<Yaml>, region: &str) -> Section {
        Section::new(list)
            .yaml_name("alarm_name")
            .resource_url(self.console_url(list, get, region))
            .raw("alarm_type")
            .raw("history_item_type")
            .raw("history_summary")
            .time("timestamp")
            .raw("history_data")
    }

    fn url_params(&self, list: &Yaml, _get: &Option<Yaml>) -> Option<Vec<(&'static str, String)>> {
        Some(vec![("alarm_name", raw(&list["alarm_name"]))])
    }
}
