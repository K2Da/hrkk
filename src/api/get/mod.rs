pub(crate) mod json_helper;
pub(crate) mod xml_helper;

use crate::error::Result;
use crate::opts::Opts;
use crate::service::prelude::Yaml;
use crate::service::GetFormat;
use crate::service::{AwsResource, GetApi};
use rusoto_core::signature::SignedRequest;

pub(crate) async fn call(resource: &dyn AwsResource, parameter: &str, opts: &Opts) -> Result<Yaml> {
    let response = super::send_request(request(resource, parameter, opts)?, opts).await?;

    let yaml = match resource.get_api() {
        Some(GetFormat::Xml { .. }) => {
            super::xml_to_yaml::convert(response.body.as_ref(), &vec![])?
        }
        Some(GetFormat::Json { .. }) => super::json_to_yaml::convert(response.body.as_ref())?,
        _ => Yaml::BadValue,
    };

    if opts.debug {
        crate::api::file::store_yaml(&yaml, "get")?;
    }

    Ok(yaml.clone())
}

fn request(resource: &dyn AwsResource, parameter: &str, opts: &Opts) -> Result<SignedRequest> {
    match &resource.info().get_api {
        Some(GetApi {
            format: GetFormat::Xml(xml_api),
            ..
        }) => xml_helper::request(opts, parameter, xml_api),
        Some(GetApi {
            format: GetFormat::Json(json_api),
            ..
        }) => json_helper::request(opts, parameter, json_api),
        _ => panic!("unknown request type"),
    }
}
