use crate::service::prelude::*;

#[derive(Serialize)]
pub(crate) struct Resource {
    info: Info,
}

pub(crate) fn new() -> Resource {
    Resource {
        info: Info {
            sub_command: Some(SubCommand::Iam {
                command: Iam::MfaDevice,
            }),
            key_attribute: Some("serial_number"),
            service_name: "iam",
            resource_type_name: "mfa_device",
            header: vec!["serial no", "user name"],
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
                    params: vec![("Action", "ListMFADevices"), ("Version", "2010-05-08")],
                    region: Some(Region::UsEast1),
                }),
                document: DocumentUrl("IAM/latest/APIReference/API_ListMFADevices.html"),
            },
            get_api: None,
            resource_url: Some(Global(
                "iam/home?#/users/{user_name}?section=security_credentials",
            )),
        },
    }
}

impl AwsResource for Resource {
    fn info(&self) -> &Info {
        &self.info
    }

    fn list_and_next_token(&self, yaml: &Yaml) -> (ResourceList, Option<String>) {
        (
            make_resource_list(self, &yaml["list_mfa_devices_result"]["mfa_devices"]),
            None,
        )
    }

    fn line(&self, list: &Yaml, _get: &Option<Yaml>) -> Vec<String> {
        vec![raw(&list["serial_number"]), raw(&list["user_name"])]
    }

    fn detail(&self, list: &Yaml, get: &Option<Yaml>, region: &str) -> Section {
        Section::new(list)
            .yaml_name("serial_number")
            .resource_url(self.console_url(list, get, region))
            .raw("user_name")
            .time("enable_date")
    }

    fn url_params(&self, list: &Yaml, _get: &Option<Yaml>) -> Option<Vec<ParamSet>> {
        Some(vec![("user_name", raw(&list["user_name"]), true)])
    }
}
