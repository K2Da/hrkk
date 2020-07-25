use crate::service::prelude::*;
use serde_json::{Map, Value};

pub(crate) fn request(opts: &Opts, parameter: &str, json_api: &GetJson) -> Result<SignedRequest> {
    let mut map = Map::<String, Value>::new();

    if let Some(parameter_name) = json_api.parameter_name {
        map.insert(
            parameter_name.to_string(),
            Value::String(parameter.to_owned()),
        );
    }

    let encoded = Value::Object(map);

    let mut path = json_api.path.0.to_string();
    if let Some(place_holder) = json_api.path.1 {
        path = path.replace(&("{".to_string() + place_holder + "}"), parameter);
    }

    let mut request = SignedRequest::new(
        json_api.method.to_str(),
        json_api.service_name,
        &opts.region()?,
        &path,
    );

    request.set_content_type("application/x-amz-json-1.1".to_owned());
    if let Some(target) = json_api.target {
        request.add_header("x-amz-target", target);
    }
    request.set_payload(Some(encoded.to_string()));
    Ok(request)
}
