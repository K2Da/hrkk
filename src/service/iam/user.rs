use crate::service::prelude::*;

#[derive(Serialize)]
pub(crate) struct Resource {
    info: Info,
}

pub(crate) fn new() -> Resource {
    Resource {
        info: Info {
            sub_command: Some(SubCommand::Iam { command: Iam::User }),
            key_attribute: Some("user_id"),
            service_name: "iam",
            resource_type_name: "user",
            header: vec!["id", "name"],
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
                    params: vec![("Action", "ListUsers"), ("Version", "2010-05-08")],
                    region: Some(Region::UsEast1),
                }),
                document: "https://docs.aws.amazon.com/IAM/latest/APIReference/API_ListUsers.html",
            },
            get_api: None,
            resource_url: Some(Global("iam/home?#/users/{user_name}")),
        },
    }
}

impl AwsResource for Resource {
    fn info(&self) -> &Info {
        &self.info
    }

    fn list_and_next_token(&self, yaml: &Yaml) -> (ResourceList, Option<String>) {
        (
            make_resource_list(self, &yaml["list_users_result"]["users"]),
            None,
        )
    }

    fn line(&self, list: &Yaml, _get: &Option<Yaml>) -> Vec<String> {
        vec![show::raw(&list["user_id"]), show::raw(&list["user_name"])]
    }

    fn detail(&self, list: &Yaml, get: &Option<Yaml>, region: &str) -> Section {
        Section::new(list)
            .yaml_name("user_name")
            .resource_url(self.console_url(list, get, region))
            .raw("user_id")
            .raw("arn")
            .raw("path")
            .time("password_last_used")
            .time("create_date")
    }

    fn url_params(&self, list: &Yaml, _get: &Option<Yaml>) -> Option<Vec<(&'static str, String)>> {
        Some(vec![("user_name", show::raw(&list["user_name"]))])
    }
}
