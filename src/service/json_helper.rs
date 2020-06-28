use crate::error::Error::*;
use crate::service::prelude::*;
use serde_json::Value;

pub(crate) fn request(
    opts: &Opts,
    next_token: Option<String>,
    parameter: &Option<String>,
    resource: &dyn AwsResource,
) -> Result<SignedRequest> {
    if let ApiType::Json {
        service_name,
        target,
        json,
        limit_name,
        token_name,
        parameter_name,
    } = &resource.info().api_type
    {
        if let Value::Object(map) = json {
            let mut map = map.clone();

            map.insert(
                limit_name.to_string(),
                Value::Number(
                    serde_json::Number::from_f64(resource.info().max_limit as f64).unwrap(),
                ),
            );

            if let Some(next_token) = next_token {
                map.insert(token_name.to_string(), Value::String(next_token));
            }

            if let Some(parameter) = parameter {
                if let Some(parameter_name) = parameter_name {
                    map.insert(
                        parameter_name.to_string(),
                        Value::String(parameter.to_owned()),
                    );
                }
            }

            let encoded = Value::Object(map);

            let mut request = SignedRequest::new("POST", service_name, &opts.region()?, "/");
            request.set_content_type("application/x-amz-json-1.1".to_owned());
            request.add_header("x-amz-target", target);
            request.set_payload(Some(encoded.to_string()));
            Ok(request)
        } else {
            Err(SettingError("request json is not a map.".to_string()))
        }
    } else {
        panic!()
    }
}

pub(crate) fn make_vec(yaml: &Yaml, root: &str) -> (Vec<Yaml>, Option<String>) {
    if let Yaml::Array(groups) = &yaml[root] {
        return (groups.clone(), next_token(&yaml));
    }

    (vec![], next_token(&yaml))
}
