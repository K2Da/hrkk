use crate::service::prelude::*;
use rusoto_core::param::{Params, ServiceParams};

pub(crate) fn request(
    opts: &Opts,
    next_token: Option<String>,
    parameter: &Option<String>,
    xml_api: &XmlListApi,
) -> Result<SignedRequest> {
    let region = if let Some(region) = xml_api.region.clone() {
        region
    } else {
        opts.region()?
    };

    let mut path = xml_api.path.to_string();
    if let Some(parameter) = parameter {
        if let Some(place_holder) = xml_api.path_place_holder {
            path = path.replace(place_holder, parameter);
        }
    }

    let mut request = SignedRequest::new(
        match xml_api.method {
            Method::Get => "GET",
            Method::Post => "POST",
        },
        xml_api.service_name,
        &region,
        &path,
    );

    let mut params = Params::new();

    if let Some(Limit {
        name: parameter_name,
        max: max_limit,
    }) = xml_api.limit
    {
        params.put(parameter_name, max_limit);
    }

    if let Some(next_token) = next_token {
        params.put(xml_api.token_name, next_token);
    }

    for (name, value) in &xml_api.params {
        params.put(name, value);
    }

    match xml_api.method {
        Method::Get => request.set_params(params),
        Method::Post => {
            request.set_payload(Some(serde_urlencoded::to_string(&params)?));
            request.set_content_type("application/x-www-form-urlencoded".to_owned());
        }
    }

    Ok(request)
}
