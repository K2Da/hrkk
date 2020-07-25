use crate::service::prelude::*;

#[derive(Serialize)]
pub(crate) struct Resource {
    info: Info,
}

pub(crate) fn new() -> Resource {
    Resource {
        info: Info {
            key_attribute: Some("alarm_name"),
            service_name: "cloudwatch",
            resource_type_name: "alarm_history",
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
                        ("Action", "DescribeAlarmHistory"),
                        ("Version", "2010-08-01")
                    ],
                    region: None,
                }),
                document: "https://docs.aws.amazon.com/AmazonCloudWatch/latest/APIReference/API_DescribeAlarmHistory.html",
            },
            get_api: None,
            resource_url: Some(
                ResourceUrl::Regional("cloudwatch/home?#alarmsV2:alarm/{alarm_name}")
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
            Some("next_token"),
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
        Some(vec![("alarm_name", show::raw(&list["alarm_name"]))])
    }
}
