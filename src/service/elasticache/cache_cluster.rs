use crate::service::prelude::*;

#[derive(Serialize)]
pub(crate) struct Resource {
    info: Info,
}

pub(crate) fn new() -> Resource {
    Resource {
        info: Info {
            key_attribute: "cache_cluster_id",
            service_name: "elasticache",
            resource_type_name: "cache_cluster",
            list_api: ListApi::Xml(XmlListApi {
                path: "/",
                path_place_holder: None,
                method: Method::Get,
                service_name: "elasticache",
                iteration_tag: vec!["CacheCluster", "Member"],
                limit: Some(Limit {
                    name: "MaxRecords",
                    max: 100,
                }),
                token_name: "Marker",
                params: vec![
                    ("Action", "DescribeCacheClusters"),
                    ("Version", "2015-02-02")
                ],
                region: None,
            }),
            list_api_document_url:
                "https://docs.aws.amazon.com/AmazonElastiCache/latest/APIReference/API_DescribeCacheClusters.html",
            get_api: None,
            get_api_document_url: None,
            resource_url: None,
        },
    }
}

impl AwsResource for Resource {
    fn info(&self) -> &Info {
        &self.info
    }

    fn matching_sub_command(&self) -> Option<SubCommand> {
        Some(SubCommand::Logs {
            command: LogsCommand::LogGroup,
        })
    }

    fn make_vec(&self, yaml: &Yaml) -> (ResourceList, Option<String>) {
        make_vec(
            self,
            &yaml["describe_cache_clusters_result"]["cache_clusters"],
            Some("marker"),
        )
    }

    fn header(&self) -> Vec<&'static str> {
        vec!["engine", "cache_cluster_id"]
    }

    fn line(&self, list: &Yaml, _get: &Option<Yaml>) -> Vec<String> {
        vec![
            show::raw(&list["engine"]),
            show::raw(&list["cache_cluster_id"]),
        ]
    }

    fn detail(&self, list: &Yaml, _get: &Option<Yaml>, _region: &str) -> Section {
        Section::new(list)
            .yaml_name("cache_cluster_id")
            .raw("engine")
            .raw("cache_cluster_status")
            .raw("cache_node_type")
    }
}
