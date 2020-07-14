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
                method: XmlListMethod::Post,
                service_name: "rds",
                action: Some("DescribeDBInstances"),
                version: Some("2014-10-31"),
                iteration_tag: vec!["Subnet", "DBInstance", "member"],
                limit: Some(Limit {
                    name: "MaxResults",
                    max: 100,
                }),
                params: vec![],
            }),
            get_api: None,
            list_api_document_url: "https://docs.aws.amazon.com/AmazonRDS/latest/APIReference/API_DescribeDBInstances.html",
            get_api_document_url: None,
            resource_url: Some("rds/home?#database:id={instance_id};is-cluster=false"
            ),
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
        make_vec(self, &yaml["describe_db_instances_result"]["db_instances"])
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
