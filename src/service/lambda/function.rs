use crate::service::prelude::*;

#[derive(Serialize)]
pub(crate) struct Resource {
    info: Info,
}

pub(crate) fn new() -> Resource {
    Resource {
        info: Info {
            key_attribute: "",
            service_name: "lambda",
            resource_type_name: "function",
            list_api: ListApi::Json(JsonListApi {
                method: JsonListMethod::Get {
                    path: "/2015-03-31/functions/",
                },
                service_name: "lambda",
                json: json!({ "descending": Some(true) }),
                limit_name: "MaxItems",
                token_name: "Marker",
                parameter_name: None,
                max_limit: 10000,
            }),
            get_api: None,
            list_api_document_url:
                "https://docs.aws.amazon.com/lambda/latest/dg/API_ListFunctions.html",
            get_api_document_url: None,
            resource_url: Some("lambda/home?#/functions/{function_name}"),
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
        make_vec(self, &yaml["functions"])
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
        Section::new(&list)
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