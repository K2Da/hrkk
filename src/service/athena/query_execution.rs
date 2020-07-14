use crate::service::prelude::*;

#[derive(Serialize)]
pub(crate) struct Resource {
    info: Info,
}

pub(crate) fn new() -> Resource {
    Resource {
        info: Info {
            key_attribute: "",
            service_name: "athena",
            resource_type_name: "query_execution",
            list_api: ListApi::Json(JsonListApi {
                method: JsonListMethod::Post {
                    target: "AmazonAthena.ListQueryExecutions",
                },
                service_name: "athena",
                json: json!({}),
                limit_name: "MaxResults",
                token_name: "NextToken",
                parameter_name: None,
                max_limit: 50,
            }),
            get_api: Some(GetApi::Json(JsonGetApi{
                service_name: "athena",
                target: "AmazonAthena.GetQueryExecution",
                parameter_name: "QueryExecutionId",
            })),
            list_api_document_url:
                "https://docs.aws.amazon.com/athena/latest/APIReference/API_ListQueryExecutions.html",
            get_api_document_url:
                Some("https://docs.aws.amazon.com/athena/latest/APIReference/API_GetQueryExecution.html"),
            resource_url: Some("athena/home?#query/history/{execution_id}"),
        },
    }
}

impl AwsResource for Resource {
    fn info(&self) -> &Info {
        &self.info
    }

    fn matching_sub_command(&self) -> Option<SubCommand> {
        Some(SubCommand::Athena {
            command: AthenaCommand::QueryExecution,
        })
    }

    fn make_vec(&self, yaml: &Yaml) -> (ResourceList, Option<String>) {
        make_vec(self, &yaml["query_execution_ids"])
    }

    fn header(&self) -> Vec<&'static str> {
        vec!["query execution id", "state", "completion time"]
    }

    fn line(&self, list: &Yaml, get: &Option<Yaml>) -> Vec<String> {
        match get {
            Some(get) => vec![
                show::raw(&get["query_execution"]["query_execution_id"]),
                show::raw(&get["query_execution"]["status"]["state"]),
                show::time(&get["query_execution"]["status"]["completion_date_time"]),
            ],
            None => vec![show::raw(&list), "".to_string()],
        }
    }

    fn detail(&self, list: &Yaml, get: &Option<Yaml>, region: &str) -> Section {
        match get {
            None => Section::new(list),
            Some(yaml) => Section::new(&yaml)
                .yaml_name2(("query_execution", "query_execution_id"))
                .resource_url(self.console_url(list, get, region))
                .raw2("query", ("query_execution", "query"))
                .raw3(
                    "output",
                    ("query_execution", "result_configuration", "output_location"),
                )
                .raw3("state", ("query_execution", "status", "state"))
                .time3(
                    "completion time",
                    ("query_execution", "status", "completion_date_time"),
                )
                .byte3(
                    "data scanned",
                    ("query_execution", "statistics", "data_scanned_in_bytes"),
                )
                .milli_sec3(
                    "execution sec",
                    (
                        "query_execution_detail",
                        "stats",
                        "engine_execution_time_in_millis",
                    ),
                ),
        }
    }

    fn url_params(&self, _list: &Yaml, get: &Option<Yaml>) -> Option<Vec<(&'static str, String)>> {
        if let Some(yaml) = get {
            Some(vec![(
                "execution_id",
                show::raw(&yaml["query_execution"]["query_execution_id"]),
            )])
        } else {
            None
        }
    }
}
