use crate::service::prelude::*;

#[derive(Serialize)]
pub(crate) struct Resource {
    info: Info,
}

pub(crate) fn new() -> Resource {
    Resource {
        info: Info {
            key_attribute: "instance_id",
            service_name: "athena",
            resource_type_name: "query_execution",
            api_type: ApiType::Json(JsonApi {
                service_name: "athena",
                target: "AmazonAthena.ListQueryExecutions",
                json: json!({}),
                limit_name: "MaxResults",
                token_name: "NextToken",
                parameter_name: None,
                max_limit: 50,
            }),
            document_url:
                "https://docs.aws.amazon.com/athena/latest/APIReference/API_ListQueryExecutions.html",
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

    fn make_vec(&self, yaml: &Yaml) -> (Vec<Yaml>, Option<String>) {
        let mut result = vec![];

        if let Yaml::Array(ids) = &yaml["query_execution_ids"] {
            for id in ids {
                result.push(id.clone());
            }
        }

        (result, next_token(&yaml))
    }

    fn header(&self) -> Vec<&'static str> {
        vec!["query execution id"]
    }

    fn line(&self, item: &Yaml) -> Vec<String> {
        vec![show::raw(&item)]
    }

    fn detail(&self, yaml: &Yaml) -> crate::show::Section {
        crate::show::Section::new(&yaml)
            .tag_name("tag_set", "Name")
            .raw("instance_id", "instance_id")
            .raw("instance_type", "instance_type")
            .raw("architecture", "architecture")
            .raw2("state", ("instance_state", "name"))
            .section(
                crate::show::Section::new(&yaml)
                    .string_name("tags")
                    .yaml_pairs("tag_set", ("key", "value")),
            )
            .section(
                crate::show::Section::new(&yaml)
                    .string_name("network")
                    .raw("subnet id", "subnet_id")
                    .raw("private ip address", "private_ip_address")
                    .raw2("availability zone", ("placement", "availability_zone")),
            )
            .section(
                crate::show::Section::new(&yaml)
                    .string_name("device")
                    .raw("root device type", "root_device_type")
                    .raw("root device name", "root_device_name"),
            )
    }
}
