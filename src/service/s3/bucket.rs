use crate::service::prelude::*;

#[derive(Serialize)]
pub(crate) struct Resource {
    info: Info,
}

pub(crate) fn new() -> Resource {
    Resource {
        info: Info {
            sub_command: Some(SubCommand::S3 {
                command: S3Command::Bucket,
            }),
            key_attribute: Some("name"),
            service_name: "s3",
            resource_type_name: "bucket",
            header: vec!["name", "creation date"],
            list_api: ListApi {
                format: ListFormat::Xml(ListXml {
                    path: ("/", None),
                    method: Method::Get,
                    service_name: "s3",
                    iteration_tag: vec!["Bucket"],
                    limit: None,
                    token_name: "",
                    params: vec![],
                    region: None,
                }),
                document: DocumentUrl("AmazonS3/latest/API/API_ListBuckets.html"),
            },
            get_api: None,
            resource_url: Some(Global("s3/buckets/{bucket_name}/")),
        },
    }
}

impl AwsResource for Resource {
    fn info(&self) -> &Info {
        &self.info
    }

    fn list_and_next_token(&self, yaml: &Yaml) -> (ResourceList, Option<String>) {
        (make_resource_list(self, &yaml["buckets"]), None)
    }

    fn line(&self, list: &Yaml, _get: &Option<Yaml>) -> Vec<String> {
        vec![raw(&list["name"]), time(&list["creation_date"])]
    }

    fn detail(&self, list: &Yaml, get: &Option<Yaml>, region: &str) -> Section {
        Section::new(list)
            .yaml_name("name")
            .resource_url(self.console_url(list, get, region))
            .time("creation_date")
    }

    fn url_params(&self, list: &Yaml, _get: &Option<Yaml>) -> Option<Vec<ParamSet>> {
        Some(vec![("bucket_name", raw(&list["name"]), true)])
    }
}
