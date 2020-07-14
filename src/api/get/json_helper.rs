use crate::service::prelude::*;
use serde_json::{Map, Value};

pub(crate) fn request(
    opts: &Opts,
    parameter: &str,
    json_api: &JsonGetApi,
) -> Result<SignedRequest> {
    let mut map = Map::<String, Value>::new();

    map.insert(
        json_api.parameter_name.to_string(),
        Value::String(parameter.to_owned()),
    );

    let encoded = Value::Object(map);

    let mut request = SignedRequest::new("POST", json_api.service_name, &opts.region()?, "/");
    request.set_content_type("application/x-amz-json-1.1".to_owned());
    request.add_header("x-amz-target", json_api.target);
    request.set_payload(Some(encoded.to_string()));
    Ok(request)
}
