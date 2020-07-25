use crate::service::prelude::*;

#[derive(Serialize)]
pub(crate) struct Resource {
    info: Info,
}

pub(crate) fn new() -> Resource {
    Resource {
        info: Info {
            key_attribute: Some("function_name"),
            service_name: "lambda",
            resource_type_name: "function",
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
                document: "https://docs.aws.amazon.com/lambda/latest/dg/API_ListFunctions.html",
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

    fn matching_sub_command(&self) -> Option<SubCommand> {
        Some(SubCommand::Lambda {
            command: LambdaCommand::Function,
        })
    }

    fn make_vec(&self, yaml: &Yaml) -> (ResourceList, Option<String>) {
        make_vec(self, &yaml["functions"], Some("next_marker"))
    }

    fn header(&self) -> Vec<&'static str> {
        vec!["name", "runtime"]
    }

    fn line(&self, list: &Yaml, _get: &Option<Yaml>) -> Vec<String> {
        vec![
            show::raw(&list["function_name"]),
            show::raw(&list["runtime"]),
        ]
    }

    fn detail(&self, list: &Yaml, get: &Option<Yaml>, region: &str) -> Section {
        Section::new(list)
            .yaml_name("function_name")
            .resource_url(self.console_url(list, get, region))
            .raw("runtime")
            .raw("memory_size")
            .raw("role")
    }

    fn url_params(&self, list: &Yaml, _get: &Option<Yaml>) -> Option<Vec<(&'static str, String)>> {
        Some(vec![("function_name", show::raw(&list["function_name"]))])
    }
}
