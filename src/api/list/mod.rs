pub(crate) mod json_helper;
pub(crate) mod xml_helper;

use crate::error::Result;
use crate::opts::Opts;
use crate::service::prelude::Yaml;
use crate::service::AwsResource;
use crate::service::{ApiType, XmlApi};
use rusoto_core::signature::SignedRequest;

pub(crate) async fn call(
    resource: &dyn AwsResource,
    parameter: &Option<String>,
    opts: &Opts,
    next_token: Option<String>,
) -> Result<(Vec<Yaml>, Option<String>)> {
    let response =
        super::send_request(request(resource, parameter, opts, next_token)?, opts).await?;

    let yaml = match resource.response_type() {
        ApiType::Xml(XmlApi { iteration_tag, .. }) => {
            super::xml_to_yaml::convert(response.body.as_ref(), &iteration_tag)?
        }
        ApiType::Json { .. } => super::json_to_yaml::convert(response.body.as_ref())?,
    };

    Ok(resource.make_vec(&yaml))
}

fn request(
    resource: &dyn AwsResource,
    parameter: &Option<String>,
    opts: &Opts,
    next_token: Option<String>,
) -> Result<SignedRequest> {
    match &resource.info().api_type {
        ApiType::Xml(xml_api) => xml_helper::request(opts, next_token, xml_api),
        ApiType::Json(json_api) => json_helper::request(opts, next_token, parameter, json_api),
    }
}
