use crate::service::prelude::*;

#[derive(Serialize)]
pub(crate) struct Resource {
    info: Info,
}

pub(crate) fn new() -> Resource {
    Resource {
        info: Info {
            sub_command: Some(SubCommand::Lambda {
                command: Lambda::Function,
            }),
            key_attribute: Some("function_name"),
            service_name: "lambda",
            resource_type_name: "function",
            header: vec!["name", "runtime"],
            list_api: ListApi {
                format: ListFormat::Json(ListJson {
                    method: JsonListMethod::Get {
                        path: "/2015-03-31/functions/",
                    },
                    service_name: "lambda",
                    json: json!({ "descending": Some(true) }),
                    limit: Some(Limit {
                        name: "MaxItems",
                        max: 10000,
                    }),
                    token_name: Some("Marker"),
                    parameter_name: None,
                }),
                document: DocumentUrl("lambda/latest/dg/API_ListFunctions.html"),
            },
            get_api: None,
            resource_url: Some(Regional("lambda/home?#/functions/{function_name}")),
        },
    }
}

impl AwsResource for Resource {
    fn info(&self) -> &Info {
        &self.info
    }

    fn list_and_next_token(&self, yaml: &Yaml) -> (ResourceList, Option<String>) {
        (
            make_resource_list(self, &yaml["functions"]),
            next_token(&yaml, Some("next_marker")),
        )
    }

    fn line(&self, list: &Yaml, _get: &Option<Yaml>) -> Vec<String> {
        vec![raw(&list["function_name"]), raw(&list["runtime"])]
    }

    fn detail(&self, list: &Yaml, get: &Option<Yaml>, region: &str) -> Section {
        Section::new(list)
            .yaml_name("function_name")
            .resource_url(self.console_url(list, get, region))
            .raw("runtime")
            .raw("memory_size")
            .raw("role")
    }

    fn url_params(&self, list: &Yaml, _get: &Option<Yaml>) -> Option<Vec<ParamSet>> {
        Some(vec![("function_name", raw(&list["function_name"]), true)])
    }
}
