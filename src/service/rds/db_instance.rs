use crate::service::prelude::*;

#[derive(Serialize)]
pub(crate) struct Resource {
    info: Info,
}

pub(crate) fn new() -> Resource {
    Resource {
        info: Info {
            sub_command: Some(SubCommand::Rds { command: RdsCommand::DbInstance }),
            key_attribute: Some("db_instance_identifier"),
            service_name: "rds",
            resource_type_name: "db_instance",
            header: vec!["status", "identifier"],
            list_api: ListApi {
                format: ListFormat::Xml(ListXml {
                    path: ("/", None),
                    method: Method::Post,
                    service_name: "rds",
                    iteration_tag: vec!["Subnet", "DBInstance", "member"],
                    limit: Some(Limit {
                        name: "MaxResults",
                        max: 100,
                    }),
                    token_name: "NextToken",
                    params: vec![
                        ("Action", "DescribeDBInstances"),
                        ("Version", "2014-10-31"),
                    ],
                    region: None,
                }),
                document: "https://docs.aws.amazon.com/AmazonRDS/latest/APIReference/API_DescribeDBInstances.html",
            },
            get_api: None,
            resource_url: Some(Regional("rds/home?#database:id={instance_id};is-cluster=false")),
        },
    }
}

impl AwsResource for Resource {
    fn info(&self) -> &Info {
        &self.info
    }

    fn list_and_next_token(&self, yaml: &Yaml) -> (ResourceList, Option<String>) {
        (
            make_resource_list(self, &yaml["describe_db_instances_result"]["db_instances"]),
            next_token(&yaml["describe_db_instances_result"], Some("marker")),
        )
    }

    fn line(&self, list: &Yaml, _get: &Option<Yaml>) -> Vec<String> {
        vec![
            raw(&list["db_instance_status"]),
            raw(&list["db_instance_identifier"]),
        ]
    }

    fn detail(&self, list: &Yaml, get: &Option<Yaml>, region: &str) -> Section {
        Section::new(list)
            .yaml_name("db_instance_identifier")
            .resource_url(self.console_url(list, get, region))
            .raw_n("status", &["db_instance_status"])
            .raw_n("cluster", &["db_cluster_identifier"])
            .raw("engine")
            .raw("engine_version")
            .raw_n("class", &["db_instance_class"])
            .raw("availability_zone")
            .raw("multi_az")
            .time("instance_create_time")
            .raw("preferred_maintenance_window")
    }

    fn url_params(&self, list: &Yaml, _get: &Option<Yaml>) -> Option<Vec<(&'static str, String)>> {
        Some(vec![("instance_id", raw(&list["db_instance_identifier"]))])
    }
}
