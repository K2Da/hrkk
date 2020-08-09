use crate::service::prelude::*;

#[derive(Serialize)]
pub(crate) struct Resource {
    info: Info,
}

pub(crate) fn new() -> Resource {
    Resource {
        info: Info {
            sub_command: Some(SubCommand::Iam {
                command: Iam::Policy,
            }),
            key_attribute: Some("policy_id"),
            service_name: "iam",
            resource_type_name: "policy",
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
                    params: vec![("Action", "ListPolicies"), ("Version", "2010-05-08")],
                    region: Some(Region::UsEast1),
                }),
                document: DocumentUrl("IAM/latest/APIReference/API_ListPolicies.html"),
            },
            get_api: None,
            resource_url: Some(Global("iam/home?#/policies/{arn}")),
        },
    }
}

impl AwsResource for Resource {
    fn info(&self) -> &Info {
        &self.info
    }

    fn list_and_next_token(&self, yaml: &Yaml) -> (ResourceList, Option<String>) {
        (
            make_resource_list(self, &yaml["list_policies_result"]["policies"]),
            None,
        )
    }

    fn line(&self, list: &Yaml, _get: &Option<Yaml>) -> Vec<String> {
        vec![
            raw(&list["policy_id"]),
            raw(&list["path"]),
            raw(&list["policy_name"]),
        ]
    }

    fn detail(&self, list: &Yaml, get: &Option<Yaml>, region: &str) -> Section {
        Section::new(list)
            .yaml_name("policy_name")
            .resource_url(self.console_url(list, get, region))
            .raw("path")
            .raw("policy_id")
            .raw("arn")
            .raw("attachment_count")
            .time("create_date")
            .time("update_date")
    }

    fn url_params(&self, list: &Yaml, _get: &Option<Yaml>) -> Option<Vec<ParamSet>> {
        Some(vec![("arn", raw(&list["arn"]), false)])
    }
}
