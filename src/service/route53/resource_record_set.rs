use crate::service::prelude::*;

#[derive(Serialize)]
pub(crate) struct Resource {
    info: Info,
}

pub(crate) fn new() -> Resource {
    Resource {
        info: Info {
            key_attribute: "name",
            service_name: "route53",
            resource_type_name: "resource_record_set",
            list_api: ListApi::Xml(XmlListApi {
                path: "/2013-04-01/hostedzone/{zone_id}/rrset",
                path_place_holder: Some("{zone_id}"),
                method: XmlListMethod::Get,
                service_name: "route53",
                iteration_tag: vec!["ResourceRecordSet", "ResourceRecord"],
                limit: Some(Limit {
                    name: "maxitems",
                    max: 100,
                }),
                token_name: "NextRecordName",
                params: vec![],
                region: Some(Region::UsEast1),
            }),
            get_api: None,

            list_api_document_url:
                "https://docs.aws.amazon.com/Route53/latest/APIReference/API_ListResourceRecordSets.html",
            get_api_document_url: None,
            resource_url: None,
        },
    }
}

impl AwsResource for Resource {
    fn info(&self) -> &Info {
        &self.info
    }

    fn matching_sub_command(&self) -> Option<SubCommand> {
        None
    }

    fn take_command(&self, sub_command: &SubCommand, opts: &Opts) -> Result<ExecuteTarget> {
        if let SubCommand::Route53 {
            command: Route53Command::ResourceRecordSet { zone_id },
        } = sub_command
        {
            match zone_id {
                Some(id) => Ok(ExecuteTarget::ExecuteThis {
                    parameter: Some(id.clone()),
                }),
                None => Ok(self.without_param(opts)),
            }
        } else {
            Ok(ExecuteTarget::Null)
        }
    }

    fn without_param(&self, _opts: &Opts) -> ExecuteTarget {
        ExecuteTarget::ParameterFromResource {
            param_resource: resource_by_name("route53_hosted_zone"),
        }
    }

    fn make_vec(&self, yaml: &Yaml) -> (ResourceList, Option<String>) {
        make_vec(self, &yaml["resource_record_sets"], "marker")
    }

    fn header(&self) -> Vec<&'static str> {
        vec!["name", "type"]
    }

    fn line(&self, list: &Yaml, _get: &Option<Yaml>) -> Vec<String> {
        vec![show::raw(&list["name"]), show::raw(&list["type"])]
    }

    fn detail(&self, list: &Yaml, _get: &Option<Yaml>, _region: &str) -> Section {
        Section::new(&list).yaml_name("name").raw("type").raw("ttl")
    }
}