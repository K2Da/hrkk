use crate::service::prelude::*;

#[derive(Serialize)]
pub(crate) struct Resource {
    info: Info,
}

pub(crate) fn new() -> Resource {
    Resource {
        info: Info {
            key_attribute: Some("metric_name"),
            service_name: "cloudwatch",
            resource_type_name: "metric",
            list_api: ListApi {
                format: ListFormat::Xml(ListXml {
                    path: ("/", None),
                    method: Method::Post,
                    service_name: "monitoring",
                    iteration_tag: vec!["member"],
                    limit: None,
                    token_name: "NextToken",
                    params: vec![
                        ("Action", "ListMetrics"),
                        ("Version", "2010-08-01"),
                    ],
                    region: None,
                }),
                document: "https://docs.aws.amazon.com/AmazonCloudWatch/latest/APIReference/API_ListMetrics.html",
            },
            get_api: None,
            resource_url: None,
        },
    }
}

impl AwsResource for Resource {
    fn info(&self) -> &Info {
        &self.info
    }

    fn matching_sub_command(&self) -> Option<SubCommand> {
        Some(SubCommand::Cloudwatch {
            command: CloudwatchCommand::Metric,
        })
    }

    fn list_and_next_token(&self, yaml: &Yaml) -> (ResourceList, Option<String>) {
        (
            make_resource_list(self, &yaml["list_metrics_result"]["metrics"]),
            next_token(&yaml["list_metrics_result"], Some("next_token")),
        )
    }

    fn header(&self) -> Vec<&'static str> {
        vec!["name space", "name"]
    }

    fn line(&self, list: &Yaml, _get: &Option<Yaml>) -> Vec<String> {
        vec![
            show::raw(&list["namespace"]),
            show::raw(&list["metric_name"]),
        ]
    }

    fn detail(&self, yaml: &Yaml, _get_yaml: &Option<Yaml>, _region: &str) -> Section {
        Section::new(&yaml)
            .yaml_name("metric_name")
            .raw("namespace")
            .section(
                Section::new(&yaml)
                    .string_name("dimensions")
                    .yaml_pairs("dimensions", ("name", "value")),
            )
    }
}
