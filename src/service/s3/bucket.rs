use crate::service::prelude::*;

#[derive(Serialize)]
pub(crate) struct Resource {
    info: Info,
}

pub(crate) fn new() -> Resource {
    Resource {
        info: Info {
            key_attribute: "name",
            service_name: "s3",
            resource_type_name: "bucket",
            list_api: ListApi::Xml(XmlListApi {
                method: XmlListMethod::Get,
                service_name: "s3",
                action: None,
                version: None,
                iteration_tag: vec!["Bucket"],
                limit: None,
                params: vec![],
            }),
            list_api_document_url:
                "https://docs.aws.amazon.com/ja_jp/AmazonS3/latest/API/API_ListBuckets.html",
            get_api: None,
            get_api_document_url: None,
            resource_url: Some("s3/buckets/{bucket_name}/"),
        },
    }
}

impl AwsResource for Resource {
    fn info(&self) -> &Info {
        &self.info
    }

    fn matching_sub_command(&self) -> Option<SubCommand> {
        Some(SubCommand::S3 {
            command: S3Command::Bucket,
        })
    }

    fn make_vec(&self, yaml: &Yaml) -> (ResourceList, Option<String>) {
        make_vec(self, &yaml["buckets"])
    }

    fn header(&self) -> Vec<&'static str> {
        vec!["name", "creation date"]
    }

    fn line(&self, list: &Yaml, _get: &Option<Yaml>) -> Vec<String> {
        vec![show::raw(&list["name"]), show::time(&list["creation_date"])]
    }

    fn detail(&self, list: &Yaml, get: &Option<Yaml>, region: &str) -> Section {
        Section::new(&list)
            .yaml_name("name")
            .resource_url(self.console_url(list, get, region))
            .time("creation_date")
    }

    fn url_params(&self, list: &Yaml, _get: &Option<Yaml>) -> Option<Vec<(&'static str, String)>> {
        Some(vec![("bucket_name", show::raw(&list["name"]))])
    }
}