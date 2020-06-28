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
            api_type: ApiType::Xml {
                service_name: "rds",
                action: "DescribeDBInstances",
                version: "2014-10-31",
                limit_name: "MaxResults",
                iteration_tag: vec!["Subnet", "DBInstance"],
            },
            document_url: "https://docs.aws.amazon.com/AmazonRDS/latest/APIReference/API_DescribeDBInstances.html",
            max_limit: 100,
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

    fn make_vec(&self, yaml: &Yaml) -> (Vec<Yaml>, Option<String>) {
        let mut arr = vec![];
        let yaml = &yaml["describe_db_instances_result"];

        if let Yaml::Array(items) = &yaml["db_instances"] {
            arr.append(&mut items.clone());
        }

        (arr, next_token(&yaml))
    }

    fn header(&self) -> Vec<&'static str> {
        vec!["status", "identifier"]
    }

    fn line(&self, item: &Yaml) -> Vec<String> {
        vec![
            show::raw(&item["db_instance_status"]),
            show::raw(&item["db_instance_identifier"]),
        ]
    }

    fn detail(&self, yaml: &Yaml) -> crate::show::Section {
        crate::show::Section::new(&yaml)
            .yaml_name("db_instance_identifier")
            .raw("status", "db_instance_status")
            .raw("cluster", "db_cluster_identifier")
            .raw("engine", "engine")
            .raw("engine version", "engine_version")
            .raw("class", "db_instance_class")
            .raw("az", "availability_zone")
            .raw("multi az", "multi_az")
            .time("create time", "instance_create_time")
            .raw(
                "preferred maintenance window",
                "preferred_maintenance_window",
            )
    }
}
