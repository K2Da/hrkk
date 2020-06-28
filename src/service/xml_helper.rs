use crate::service::prelude::*;
use rusoto_core::param::{Params, ServiceParams};

pub(crate) fn request(
    opts: &Opts,
    next_token: Option<String>,
    resource: &dyn AwsResource,
) -> Result<SignedRequest> {
    if let ApiType::Xml {
        service_name,
        action,
        version,
        limit_name,
        ..
    } = resource.info().api_type
    {
        let mut request = SignedRequest::new("POST", service_name, &opts.region()?, "/");
        let mut params = Params::new();

        params.put("Action", action);
        params.put("Version", version);

        params.put(limit_name, resource.info().max_limit);

        if let Some(next_token) = next_token {
            params.put("NextToken", next_token);
        }

        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        Ok(request)
    } else {
        panic!()
    }
}
