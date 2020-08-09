use crate::service::prelude::*;

#[derive(Serialize)]
pub(crate) struct Resource {
    info: Info,
}

pub(crate) fn new() -> Resource {
    Resource {
        info: Info {
            sub_command: Some(SubCommand::Elastictranscoder {
                command: Elastictranscoder::Pipeline,
            }),
            key_attribute: Some("name"),
            service_name: "elastictranscoder",
            resource_type_name: "pipeline",
            header: vec!["id", "name"],
            list_api: ListApi {
                format: ListFormat::Json(ListJson {
                    method: JsonListMethod::Get {
                        path: "/2012-09-25/pipelines",
                    },
                    service_name: "elastictranscoder",
                    json: json!({}),
                    limit: None,
                    token_name: Some("PageToken"),
                    parameter_name: None,
                }),
                document: DocumentUrl(
                    "elastictranscoder/latest/developerguide/list-pipelines.html",
                ),
            },
            get_api: None,
            resource_url: Some(Regional("elastictranscoder/home?#pipeline-details:{id}")),
        },
    }
}

impl AwsResource for Resource {
    fn info(&self) -> &Info {
        &self.info
    }

    fn list_and_next_token(&self, yaml: &Yaml) -> (ResourceList, Option<String>) {
        (
            make_resource_list(self, &yaml["pipelines"]),
            next_token(&yaml, Some("next_page_token")),
        )
    }

    fn line(&self, list: &Yaml, _get: &Option<Yaml>) -> Vec<String> {
        vec![raw(&list["id"]), raw(&list["name"])]
    }

    fn detail(&self, list: &Yaml, get: &Option<Yaml>, region: &str) -> Section {
        Section::new(list)
            .yaml_name_n(&["name"])
            .resource_url(self.console_url(list, get, region))
            .raw("id")
            .raw("arn")
            .section(
                Section::new(&list["content_config"])
                    .string_name("bucket")
                    .raw("bucket")
                    .raw("storage_class"),
            )
    }

    fn url_params(&self, list: &Yaml, _get: &Option<Yaml>) -> Option<Vec<ParamSet>> {
        Some(vec![("id", raw(&list["id"]), true)])
    }
}
