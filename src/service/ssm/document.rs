use crate::service::prelude::*;

#[derive(Serialize)]
pub(crate) struct Resource {
    info: Info,
}

pub(crate) fn new() -> Resource {
    Resource {
        info: Info {
            key_attribute: "name",
            service_name: "ssm",
            resource_type_name: "document",
            list_api: ListApi::Json(JsonListApi {
                method: JsonListMethod::Post {
                    target: "AmazonSSM.ListDocuments",
                },
                service_name: "ssm",
                json: json!({}),
                limit_name: "MaxResults",
                token_name: "NextToken",
                parameter_name: None,
                max_limit: 50,
            }),
            get_api: None,
            list_api_document_url: "https://docs.aws.amazon.com/systems-manager/latest/APIReference/API_ListDocuments.html",
            get_api_document_url: None,
            resource_url: Some(
                "systems-manager/documents/{name}"
            ),
        },
    }
}

impl AwsResource for Resource {
    fn info(&self) -> &Info {
        &self.info
    }

    fn matching_sub_command(&self) -> Option<SubCommand> {
        Some(SubCommand::Ssm {
            command: SsmCommand::Document,
        })
    }

    fn make_vec(&self, yaml: &Yaml) -> (ResourceList, Option<String>) {
        make_vec(self, &yaml["document_identifiers"])
    }

    fn header(&self) -> Vec<&'static str> {
        vec!["type", "name", "owner"]
    }

    fn line(&self, list: &Yaml, _get: &Option<Yaml>) -> Vec<String> {
        vec![
            show::raw(&list["document_type"]),
            show::raw(&list["name"]),
            show::raw(&list["owner"]),
        ]
    }

    fn detail(&self, list: &Yaml, get: &Option<Yaml>, region: &str) -> Section {
        Section::new(&list)
            .yaml_name("name")
            .resource_url(self.console_url(list, get, region))
            .raw("document_format")
            .raw("document_type")
            .raw("owner")
            .raw("schema_version")
            .raw("target_type")
            .section(
                Section::new(&list)
                    .string_name("platform types")
                    .raw_array("platform_types"),
            )
    }

    fn url_params(&self, list: &Yaml, _get: &Option<Yaml>) -> Option<Vec<(&'static str, String)>> {
        Some(vec![("name", show::raw(&list["name"]))])
    }
}
