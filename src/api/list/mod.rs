pub(crate) mod json_helper;
pub(crate) mod xml_helper;

use crate::error::Result;
use crate::opts::Opts;
use crate::service::prelude::*;
use crate::service::AwsResource;
use crate::service::{ListFormat, ListXml};
use rusoto_core::signature::SignedRequest;

pub(crate) async fn call(
    resource: &dyn AwsResource,
    parameter: &Option<String>,
    opts: &Opts,
    next_token: Option<String>,
) -> Result<(crate::service::ResourceList, Option<String>)> {
    let response =
        super::send_request(request(resource, parameter, opts, next_token)?, opts).await?;

    let yaml = match resource.list_api() {
        ListFormat::Xml(ListXml { iteration_tag, .. }) => {
            super::xml_to_yaml::convert(response.body.as_ref(), &iteration_tag)?
        }
        ListFormat::Json { .. } => super::json_to_yaml::convert(response.body.as_ref())?,
    };

    if opts.debug {
        crate::api::file::store_yaml(&yaml, "list")?;
    }

    Ok(resource.list_and_next_token(&yaml))
}

fn request(
    resource: &dyn AwsResource,
    parameter: &Option<String>,
    opts: &Opts,
    next_token: Option<String>,
) -> Result<SignedRequest> {
    match &resource.info().list_api.format {
        ListFormat::Xml(xml_api) => xml_helper::request(opts, next_token, parameter, xml_api),
        ListFormat::Json(json_api) => json_helper::request(opts, next_token, parameter, json_api),
    }
}

pub(crate) fn make_resource_list(
    resource: &dyn crate::service::AwsResource,
    yaml: &Yaml,
) -> ResourceList {
    if let Yaml::Array(items) = &yaml {
        return items
            .iter()
            .map(|y| (resource.line(y, &None), y.clone()))
            .collect();
    }

    vec![]
}
