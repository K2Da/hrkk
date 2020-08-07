use crate::service::prelude::*;

#[derive(Serialize)]
pub(crate) struct Resource {
    info: Info,
}

pub(crate) fn new() -> Resource {
    Resource {
        info: Info {
            sub_command: Some(SubCommand::Iam {
                command: Iam::Group,
            }),
            key_attribute: Some("group_id"),
            service_name: "iam",
            resource_type_name: "group",
            header: vec!["id", "path", "name"],
            list_api: ListApi {
                format: ListFormat::Xml(ListXml {
                    path: ("/", None),
                    method: Method::Get,
                    service_name: "iam",
                    iteration_tag: vec!["member"],
                    limit: Some(Limit {
                        name: "MaxItems",
                        max: 1000,
                    }),
                    token_name: "Marker",
                    params: vec![("Action", "ListGroups"), ("Version", "2010-05-08")],
                    region: Some(Region::UsEast1),
                }),
                document: DocumentUrl("IAM/latest/APIReference/API_ListGroups.html"),
            },
            get_api: None,
            resource_url: Some(Global("iam/home?#/groups/{group_name}")),
        },
    }
}

impl AwsResource for Resource {
    fn info(&self) -> &Info {
        &self.info
    }

    fn list_and_next_token(&self, yaml: &Yaml) -> (ResourceList, Option<String>) {
        (
            make_resource_list(self, &yaml["list_groups_result"]["groups"]),
            None,
        )
    }

    fn line(&self, list: &Yaml, _get: &Option<Yaml>) -> Vec<String> {
        vec![
            raw(&list["group_id"]),
            raw(&list["path"]),
            raw(&list["group_name"]),
        ]
    }

    fn detail(&self, list: &Yaml, get: &Option<Yaml>, region: &str) -> Section {
        Section::new(list)
            .yaml_name("group_name")
            .resource_url(self.console_url(list, get, region))
            .raw("group_id")
            .raw("path")
            .raw("arn")
    }

    fn url_params(&self, list: &Yaml, _get: &Option<Yaml>) -> Option<Vec<(&'static str, String)>> {
        Some(vec![("group_name", raw(&list["group_name"]))])
    }
}
