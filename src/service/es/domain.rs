use crate::service::prelude::*;

#[derive(Serialize)]
pub(crate) struct Resource {
    info: Info,
}

pub(crate) fn new() -> Resource {
    Resource {
        info: Info {
            sub_command: Some(SubCommand::Es { command: Es::Domain }),
            key_attribute: Some("domain_name"),
            service_name: "es",
            resource_type_name: "domain",
            header: vec!["name", "version"],
            list_api: ListApi {
                format: ListFormat::Json(ListJson {
                    method: JsonListMethod::Get {
                        path: "/2015-01-01/domain",
                    },
                    service_name: "es",
                    json: json!({}),
                    limit: None,
                    token_name: None,
                    parameter_name: None,
                }),
                document: DocumentUrl("elasticsearch-service/latest/developerguide/es-configuration-api.html#es-configuration-api-actions-listdomainnames"),
            },
            get_api: Some(GetApi {
                param_path: vec!["domain_name"],
                format: GetFormat::Json(GetJson {
                    method: Method::Get,
                    path: ("/2015-01-01/es/domain/{domain_name}", Some("domain_name")),
                    service_name: "es",
                    target: None,
                    parameter_name: None,
                }),
                document: "https://docs.aws.amazon.com/elasticsearch-service/latest/developerguide/es-configuration-api.html#es-configuration-api-actions-describeelasticsearchdomain",
            }),
            resource_url: Some(Regional("es/home?#domain:resource={domain_name};action=dashboard")),
        },
    }
}

impl AwsResource for Resource {
    fn info(&self) -> &Info {
        &self.info
    }

    fn list_and_next_token(&self, yaml: &Yaml) -> (ResourceList, Option<String>) {
        (make_resource_list(self, &yaml["domain_names"]), None)
    }

    fn line(&self, list: &Yaml, get: &Option<Yaml>) -> Vec<String> {
        match get {
            Some(get) => vec![
                raw(&get["domain_status"]["domain_name"]),
                raw(&get["domain_status"]["elasticsearch_version"]),
            ],
            None => vec![raw(&list["domain_name"]), "-".to_string()],
        }
    }

    fn detail(&self, list: &Yaml, get: &Option<Yaml>, region: &str) -> Section {
        match get {
            None => Section::new(list).yaml_name("domain_name"),
            Some(yaml) => Section::new(&yaml["domain_status"])
                .resource_url(self.console_url(list, get, region))
                .yaml_name("domain_name")
                .raw("elasticsearch_version")
                .raw("arn")
                .raw("created")
                .raw("deleted")
                .section(
                    Section::new(&yaml["domain_status"]["elasticsearch_cluster_config"])
                        .string_name("cluster config")
                        .raw("instance_type")
                        .raw("instance_count"),
                )
                .section(
                    Section::new(&yaml["domain_status"]["vpc_options"])
                        .string_name("vpc options")
                        .yaml_array("availability zones", "availability_zones"),
                ),
        }
    }

    fn url_params(&self, list: &Yaml, _get: &Option<Yaml>) -> Option<Vec<ParamSet>> {
        Some(vec![("domain_name", raw(&list["domain_name"]), true)])
    }
}
