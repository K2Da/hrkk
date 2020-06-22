use crate::service::prelude::*;

#[derive(Serialize)]
pub struct Resource {
    info: Info,
}

pub fn new() -> Resource {
    Resource {
        info: Info {
            key_attribute: "name",
            service_name: "ssm",
            resource_type_name: "document",
            api_type: ApiType::Json {
                service_name: "ssm",
                target: "AmazonSSM.ListDocuments",
                json: json!({}),
                limit_name: "MaxResults",
                token_name: "NextToken",
                parameter_name: None,
            },
            document_url: "https://docs.aws.amazon.com/systems-manager/latest/APIReference/API_ListDocuments.html",
            max_limit: 50,
        },
    }
}

impl AwsResource for Resource {
    fn info(&self) -> &Info {
        &self.info
    }

    fn matching_sub_command(&self) -> Option<SubCommand> {
        Some(SubCommand::Ssm {
            command: SsmCommand::Document,
        })
    }

    fn make_vec(&self, yaml: &Yaml) -> (Vec<Yaml>, Option<String>) {
        json_helper::make_vec(&yaml, "document_identifiers")
    }

    fn header(&self) -> Vec<&'static str> {
        vec!["type", "name", "owner"]
    }

    fn header_width(&self) -> Vec<Constraint> {
        vec![
            Constraint::Length(10),
            Constraint::Min(0),
            Constraint::Min(0),
        ]
    }

    fn line(&self, item: &Yaml) -> Vec<String> {
        vec![
            show::raw(&item["document_type"]),
            show::raw(&item["name"]),
            show::raw(&item["owner"]),
        ]
    }

    fn detail(&self, yaml: &Yaml) -> crate::show::Section {
        crate::show::Section::new(&yaml)
            .yaml_name("name")
            .raw("format", "document_format")
            .raw("type", "document_type")
            .raw("owner", "owner")
            .raw("schema version", "schema_version")
            .raw("target type", "target_type")
            .section(
                crate::show::Section::new(&yaml)
                    .string_name("platform types")
                    .raw_array("platform_types"),
            )
    }
}
