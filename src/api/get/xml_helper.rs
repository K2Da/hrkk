use crate::service::prelude::*;
use rusoto_core::param::{Params, ServiceParams};

pub(crate) fn request(opts: &Opts, parameter: &str, xml_api: &XmlGetApi) -> Result<SignedRequest> {
    let mut request = SignedRequest::new("POST", xml_api.service_name, &opts.region()?, "/");
    let mut params = Params::new();

    params.put("Action", xml_api.action);
    params.put("Version", xml_api.version);
    params.put(xml_api.parameter_name, parameter);

    request.set_payload(Some(serde_urlencoded::to_string(&params)?));
    request.set_content_type("application/x-www-form-urlencoded".to_owned());

    Ok(request)
}
