use crate::service::prelude::*;

#[derive(Serialize)]
pub(crate) struct Resource {
    info: Info,
}

pub(crate) fn new() -> Resource {
    Resource {
        info: Info {
            key_attribute: "db_instance_identifier",
            service_name: "rds",
            resource_type_name: "db_instance",
            list_api: ListApi::Xml(XmlListApi {
                path: "/",
                path_place_holder: None,
                method: XmlListMethod::Post,
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
            get_api: None,
            list_api_document_url: "https://docs.aws.amazon.com/AmazonRDS/latest/APIReference/API_DescribeDBInstances.html",
            get_api_document_url: None,
            resource_url: Some(ResourceUrl::Regional("rds/home?#database:id={instance_id};is-cluster=false")),
        },
    }
}

impl AwsResource for Resource {
    fn info(&self) -> &Info {
        &self.info
    }

    fn matching_sub_command(&self) -> Option<SubCommand> {
        Some(SubCommand::Rds {
            command: RdsCommand::DbInstance,
        })
    }

    fn make_vec(&self, yaml: &Yaml) -> (ResourceList, Option<String>) {
        make_vec(
            self,
            &yaml["describe_db_instances_result"]["db_instances"],
            "marker",
        )
    }

    fn header(&self) -> Vec<&'static str> {
        vec!["status", "identifier"]
    }

    fn line(&self, list: &Yaml, _get: &Option<Yaml>) -> Vec<String> {
        vec![
            show::raw(&list["db_instance_status"]),
            show::raw(&list["db_instance_identifier"]),
        ]
    }

    fn detail(&self, list: &Yaml, get: &Option<Yaml>, region: &str) -> Section {
        Section::new(&list)
            .yaml_name("db_instance_identifier")
            .resource_url(self.console_url(list, get, region))
            .raw1("status", "db_instance_status")
            .raw1("cluster", "db_cluster_identifier")
            .raw("engine")
            .raw("engine_version")
            .raw1("class", "db_instance_class")
            .raw("availability_zone")
            .raw("multi_az")
            .time("instance_create_time")
            .raw("preferred_maintenance_window")
    }

    fn url_params(&self, list: &Yaml, _get: &Option<Yaml>) -> Option<Vec<(&'static str, String)>> {
        Some(vec![(
            "instance_id",
            show::raw(&list["db_instance_identifier"]),
        )])
    }
}
