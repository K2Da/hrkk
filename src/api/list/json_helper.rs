use crate::service::prelude::*;
use serde_json::Value;

pub(crate) fn request(
    opts: &Opts,
    next_token: Option<String>,
    parameter: &Option<String>,
    json_api: &JsonApi,
) -> Result<SignedRequest> {
    let mut map = json_api.json_map()?;
    map.insert(
        json_api.limit_name.to_string(),
        Value::Number(serde_json::Number::from_f64(json_api.max_limit as f64).unwrap()),
    );

    if let Some(next_token) = next_token {
        map.insert(json_api.token_name.to_string(), Value::String(next_token));
    }

    if let Some(parameter) = parameter {
        if let Some(parameter_name) = json_api.parameter_name {
            map.insert(
                parameter_name.to_string(),
                Value::String(parameter.to_owned()),
            );
        }
    }

    let encoded = Value::Object(map);

    let mut request = SignedRequest::new("POST", json_api.service_name, &opts.region()?, "/");
    request.set_content_type("application/x-amz-json-1.1".to_owned());
    request.add_header("x-amz-target", json_api.target);
    request.set_payload(Some(encoded.to_string()));
    Ok(request)
}

pub(crate) fn make_vec(yaml: &Yaml, root: &str) -> (Vec<Yaml>, Option<String>) {
    if let Yaml::Array(groups) = &yaml[root] {
        return (groups.clone(), next_token(&yaml));
    }

    (vec![], next_token(&yaml))
}
