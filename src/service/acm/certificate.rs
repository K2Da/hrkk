use crate::service::prelude::*;

#[derive(Serialize)]
pub(crate) struct Resource {
    info: Info,
}

pub(crate) fn new() -> Resource {
    Resource {
        info: Info {
            sub_command: Some(SubCommand::Acm {
                command: Acm::Certificate,
            }),
            key_attribute: Some("certificate_arn"),
            service_name: "acm",
            resource_type_name: "certificate",
            header: vec!["domain name", "arn"],
            list_api: ListApi {
                format: ListFormat::Json(ListJson {
                    method: JsonListMethod::Post {
                        target: "CertificateManager.ListCertificates",
                    },
                    service_name: "acm",
                    json: json!({}),
                    limit: Some(Limit {
                        name: "MaxItems",
                        max: 1000,
                    }),
                    token_name: Some("NextToken"),
                    parameter_name: None,
                }),
                document:
                    "https://docs.aws.amazon.com/acm/latest/APIReference/API_ListCertificates.html",
            },
            get_api: Some(GetApi {
                param_path: vec!["certificate_arn"],
                format: GetFormat::Json(GetJson {
                    method: Method::Post,
                    path: ("/", None),
                    service_name: "acm",
                    target: Some("CertificateManager.GetCertificate"),
                    parameter_name: Some("CertificateArn"),
                }),
                document:
                    "https://docs.aws.amazon.com/acm/latest/APIReference/API_GetCertificate.html",
            }),
            resource_url: Some(Regional("acm/home")),
        },
    }
}

impl AwsResource for Resource {
    fn info(&self) -> &Info {
        &self.info
    }

    fn list_and_next_token(&self, yaml: &Yaml) -> (ResourceList, Option<String>) {
        (
            make_resource_list(self, &yaml["certificate_summary_list"]),
            next_token(&yaml, Some("next_token")),
        )
    }

    fn line(&self, list: &Yaml, _get: &Option<Yaml>) -> Vec<String> {
        vec![raw(&list["domain_name"]), raw(&list["certificate_arn"])]
    }

    fn detail(&self, list: &Yaml, get: &Option<Yaml>, region: &str) -> Section {
        match get {
            Some(get_yaml) => {
                let merged = merge_yamls(list, get_yaml);
                Section::new(&merged)
                    .yaml_name_n(&["list", "domain_name"])
                    .resource_url(self.console_url(list, get, region))
                    .raw_n("arn", &["list", "certificate_arn"])
                    .raw_n("certificate", &["get", "certificate"])
                    .raw_n("certificate chain", &["get", "certificate_chain"])
            }
            _ => Section::new(list)
                .yaml_name_n(&["list", "domain_name"])
                .resource_url(self.console_url(list, get, region)),
        }
    }

    fn url_params(&self, _list: &Yaml, _get: &Option<Yaml>) -> Option<Vec<(&'static str, String)>> {
        Some(vec![])
    }
}
