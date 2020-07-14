use crate::service::prelude::*;
use rusoto_core::param::{Params, ServiceParams};

pub(crate) fn request(
    opts: &Opts,
    next_token: Option<String>,
    xml_api: &XmlListApi,
) -> Result<SignedRequest> {
    let mut request = SignedRequest::new(
        match xml_api.method {
            XmlListMethod::Get => "GET",
            XmlListMethod::Post => "POST",
        },
        xml_api.service_name,
        &opts.region()?,
        "/",
    );
    let mut params = Params::new();

    if let Some(action) = xml_api.action {
        params.put("Action", action);
    }

    if let Some(version) = xml_api.version {
        params.put("Version", version);
    }

    if let Some(Limit {
        name: parameter_name,
        max: max_limit,
    }) = xml_api.limit
    {
        params.put(parameter_name, max_limit);
    }

    if let Some(next_token) = next_token {
        params.put("NextToken", next_token);
    }

    for (name, value) in &xml_api.params {
        params.put(name, value);
    }

    request.set_payload(Some(serde_urlencoded::to_string(&params)?));
    request.set_content_type("application/x-www-form-urlencoded".to_owned());

    Ok(request)
}
