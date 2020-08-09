use crate::service::prelude::*;

#[derive(Serialize)]
pub(crate) struct Resource {
    info: Info,
}

pub(crate) fn new() -> Resource {
    Resource {
        info: Info {
            sub_command: Some(SubCommand::Iam { command: Iam::Role }),
            key_attribute: Some("role_id"),
            service_name: "iam",
            resource_type_name: "role",
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
                    params: vec![("Action", "ListRoles"), ("Version", "2010-05-08")],
                    region: Some(Region::UsEast1),
                }),
                document: DocumentUrl("IAM/latest/APIReference/API_ListRoles.html"),
            },
            get_api: None,
            resource_url: Some(Global("iam/home?#/roles/{role_name}")),
        },
    }
}

impl AwsResource for Resource {
    fn info(&self) -> &Info {
        &self.info
    }

    fn list_and_next_token(&self, yaml: &Yaml) -> (ResourceList, Option<String>) {
        (
            make_resource_list(self, &yaml["list_roles_result"]["roles"]),
            None,
        )
    }

    fn line(&self, list: &Yaml, _get: &Option<Yaml>) -> Vec<String> {
        vec![
            raw(&list["role_id"]),
            raw(&list["path"]),
            raw(&list["role_name"]),
        ]
    }

    fn detail(&self, list: &Yaml, get: &Option<Yaml>, region: &str) -> Section {
        Section::new(list)
            .yaml_name("role_name")
            .resource_url(self.console_url(list, get, region))
            .raw("role_id")
            .raw("arn")
            .raw("path")
            .raw("assume_role_policy_document")
            .time("create_date")
    }

    fn url_params(&self, list: &Yaml, _get: &Option<Yaml>) -> Option<Vec<ParamSet>> {
        Some(vec![("role_name", raw(&list["role_name"]), true)])
    }
}
