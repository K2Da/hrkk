use crate::service::prelude::*;

#[derive(Serialize)]
pub(crate) struct Resource {
    info: Info,
}

pub(crate) fn new() -> Resource {
    Resource {
        info: Info {
            sub_command: Some(SubCommand::Elasticache { command: Elasticache::CacheCluster }),
            key_attribute: Some("cache_cluster_id"),
            service_name: "elasticache",
            resource_type_name: "cache_cluster",
            header: vec!["engine", "cache_cluster_id"],
            list_api: ListApi {
                format: ListFormat::Xml(ListXml {
                    path: ("/", None),
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
                document: "https://docs.aws.amazon.com/AmazonElastiCache/latest/APIReference/API_DescribeCacheClusters.html",
            },
            get_api: None,
            resource_url: None,
        },
    }
}

impl AwsResource for Resource {
    fn info(&self) -> &Info {
        &self.info
    }

    fn list_and_next_token(&self, yaml: &Yaml) -> (ResourceList, Option<String>) {
        (
            make_resource_list(
                self,
                &yaml["describe_cache_clusters_result"]["cache_clusters"],
            ),
            next_token(&yaml["describe_cache_clusters_result"], Some("marker")),
        )
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
