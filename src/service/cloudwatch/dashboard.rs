use crate::service::prelude::*;

#[derive(Serialize)]
pub(crate) struct Resource {
    info: Info,
}

pub(crate) fn new() -> Resource {
    Resource {
        info: Info {
            sub_command: Some(SubCommand::Cloudwatch { command: Cloudwatch::Metric }),
            key_attribute: Some("dashboard"),
            service_name: "cloudwatch",
            resource_type_name: "dashboard",
            header: vec!["name", "size", "modified", "body"],
            list_api: ListApi {
                format: ListFormat::Xml(ListXml {
                    path: ("/", None),
                    method: Method::Post,
                    service_name: "monitoring",
                    iteration_tag: vec!["member"],
                    limit: None,
                    token_name: "NextToken",
                    params: vec![
                        ("Action", "ListDashboards"),
                        ("Version", "2010-08-01"),
                    ],
                    region: None,
                }),
                document: "https://docs.aws.amazon.com/AmazonCloudWatch/latest/APIReference/API_ListDashboards.html",
            },
            get_api: Some(GetApi {
                param_path: vec!["dashboard_name"],
                format: GetFormat::Xml(GetXml {
                    service_name: "monitoring",
                    action: "GetDashboard",
                    version: "2010-08-01",
                    parameter_name: "DashboardName",
                }),
                document: "https://docs.aws.amazon.com/AmazonCloudWatch/latest/APIReference/API_GetDashboard.html",
            }),
            resource_url: Some(
                Regional("cloudwatch/home?#dashboards:name={dashboard_name}")
            ),
        },
    }
}

impl AwsResource for Resource {
    fn info(&self) -> &Info {
        &self.info
    }

    fn list_and_next_token(&self, yaml: &Yaml) -> (ResourceList, Option<String>) {
        (
            make_resource_list(self, &yaml["list_dashboards_result"]["dashboard_entries"]),
            next_token(&yaml["list_dashboards_result"], Some("next_token")),
        )
    }

    fn line(&self, list: &Yaml, get: &Option<Yaml>) -> Vec<String> {
        match get {
            Some(get) => vec![
                show::raw(&list["dashboard_name"]),
                show::byte(&list["size"]),
                show::time(&list["last_modified"]),
                show::raw(&get["get_dashboard_result"]["dashboard_body"]),
            ],
            None => vec![
                show::raw(&list["dashboard_name"]),
                show::byte(&list["size"]),
                show::time(&list["last_modified"]),
                "".to_owned(),
            ],
        }
    }

    fn detail(&self, list: &Yaml, get: &Option<Yaml>, region: &str) -> Section {
        match get {
            Some(get_yaml) => {
                let merged = merge_yamls(list, get_yaml);
                Section::new(&merged)
                    .yaml_name2(("list", "dashboard_name"))
                    .resource_url(self.console_url(list, get, region))
                    .byte_n("size", &["list", "size"])
                    .time_n("last modified", &["list", "last_modified"])
                    .raw_n("body", &["get", "get_dashboard_result", "dashboard_body"])
            }
            None => Section::new(list)
                .yaml_name("dashboard_name")
                .byte("size")
                .time("last_modified"),
        }
    }

    fn url_params(&self, _list: &Yaml, get: &Option<Yaml>) -> Option<Vec<(&'static str, String)>> {
        if let Some(yaml) = get {
            Some(vec![(
                "dashboard_name",
                show::raw(&yaml["list"]["dashboard_name"]),
            )])
        } else {
            None
        }
    }
}
