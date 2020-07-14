use crate::service::prelude::*;

#[derive(Serialize)]
pub(crate) struct Resource {
    info: Info,
}

pub(crate) fn new() -> Resource {
    Resource {
        info: Info {
            key_attribute: "dashboard",
            service_name: "cloudwatch",
            resource_type_name: "dashboard",
            list_api: ListApi::Xml(XmlListApi {
                method: XmlListMethod::Post,
                service_name: "monitoring",
                action: Some("ListDashboards"),
                version: Some("2010-08-01"),
                iteration_tag: vec!["member"],
                limit: None,
                params: vec![],
            }),
            list_api_document_url:
                "https://docs.aws.amazon.com/AmazonCloudWatch/latest/APIReference/API_ListDashboards.html",
            get_api: Some(GetApi::Xml(XmlGetApi {
                service_name: "monitoring",
                action: "GetDashboard",
                version: "2010-08-01",
                parameter_name: "DashboardName",
            })),
            get_api_document_url:
                Some("https://docs.aws.amazon.com/AmazonCloudWatch/latest/APIReference/API_GetDashboard.html"),
            resource_url: Some(
                "cloudwatch/home?#dashboards:name={dashboard_name}"
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
            command: CloudwatchCommand::Metric,
        })
    }

    fn make_vec(&self, yaml: &Yaml) -> (ResourceList, Option<String>) {
        make_vec(self, &yaml["list_dashboards_result"]["dashboard_entries"])
    }

    fn header(&self) -> Vec<&'static str> {
        vec!["name", "size", "modified", "body"]
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
                    .byte2("size", ("list", "size"))
                    .time2("last modified", ("list", "last_modified"))
                    .raw3("body", ("get", "get_dashboard_result", "dashboard_body"))
            }
            None => Section::new(&list)
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
