use crate::service::prelude::*;

#[derive(Serialize)]
pub(crate) struct Resource {
    info: Info,
}

pub(crate) fn new() -> Resource {
    Resource {
        info: Info {
            sub_command: Some(SubCommand::Athena { command: Athena::QueryExecution }),
            key_attribute: None,
            service_name: "athena",
            resource_type_name: "query_execution",
            header: vec!["query execution id", "state", "completion time"],
            list_api: ListApi {
                format: ListFormat::Json(ListJson {
                    method: JsonListMethod::Post { target: "AmazonAthena.ListQueryExecutions", },
                    service_name: "athena",
                    json: json!({}),
                    limit: Some(Limit { name: "MaxResults", max: 50 }),
                    token_name: Some("NextToken"),
                    parameter_name: None,
                }),
                document: DocumentUrl("athena/latest/APIReference/API_ListQueryExecutions.html"),
            },
            get_api: Some(GetApi {
                param_path: vec![],
                format: GetFormat::Json(GetJson {
                    method: Method::Post,
                    path: ("/", None),
                    service_name: "athena",
                    target: Some("AmazonAthena.GetQueryExecution"),
                    parameter_name: Some("QueryExecutionId"),
                }),
                document: "https://docs.aws.amazon.com/athena/latest/APIReference/API_GetQueryExecution.html",
            }),
            resource_url: Some(Regional("athena/home?#query/history/{execution_id}")),
        },
    }
}

impl AwsResource for Resource {
    fn info(&self) -> &Info {
        &self.info
    }

    fn list_and_next_token(&self, yaml: &Yaml) -> (ResourceList, Option<String>) {
        (
            make_resource_list(self, &yaml["query_execution_ids"]),
            next_token(&yaml, Some("next_token")),
        )
    }

    fn line(&self, list: &Yaml, get: &Option<Yaml>) -> Vec<String> {
        match get {
            Some(get) => vec![
                raw(&get["query_execution"]["query_execution_id"]),
                raw(&get["query_execution"]["status"]["state"]),
                time(&get["query_execution"]["status"]["completion_date_time"]),
            ],
            None => vec![raw(&list), "".to_string()],
        }
    }

    fn detail(&self, list: &Yaml, get: &Option<Yaml>, region: &str) -> Section {
        match get {
            None => Section::new(list),
            Some(yaml) => Section::new(&yaml)
                .yaml_name_n(&["query_execution", "query_execution_id"])
                .resource_url(self.console_url(list, get, region))
                .raw_n("query", &["query_execution", "query"])
                .raw_n(
                    "output",
                    &["query_execution", "result_configuration", "output_location"],
                )
                .raw_n("state", &["query_execution", "status", "state"])
                .time_n(
                    "completion time",
                    &["query_execution", "status", "completion_date_time"],
                )
                .byte_n(
                    "data scanned",
                    &["query_execution", "statistics", "data_scanned_in_bytes"],
                )
                .milli_sec_n(
                    "execution sec",
                    &[
                        "query_execution_detail",
                        "stats",
                        "engine_execution_time_in_millis",
                    ],
                ),
        }
    }

    fn url_params(&self, _list: &Yaml, get: &Option<Yaml>) -> Option<Vec<(&'static str, String)>> {
        if let Some(yaml) = get {
            Some(vec![(
                "execution_id",
                raw(&yaml["query_execution"]["query_execution_id"]),
            )])
        } else {
            None
        }
    }
}
