pub(crate) mod file;
pub(crate) mod json_to_yaml;
pub(crate) mod list;
pub(crate) mod xml_to_yaml;

use crate::error::Error::*;
use crate::error::Result;
use crate::opts::Opts;
use rusoto_core::request::BufferedHttpResponse;
use rusoto_core::signature::SignedRequest;
use rusoto_core::{Client, HttpClient};
use rusoto_credential::ChainProvider;

async fn send_request(request: SignedRequest, opts: &Opts) -> Result<BufferedHttpResponse> {
    let mut response = Client::new_with(ChainProvider::default(), HttpClient::new()?)
        .sign_and_dispatch(request)
        .await
        .map_err(|e| RusotoError(format!("{:?}", e)))?;

    let response = response
        .buffer()
        .await
        .map_err(|e| RusotoError(format!("{}", e)))?;

    if !response.status.is_success() {
        return Err(RusotoError(
            String::from_utf8(response.body.as_ref().to_vec()).unwrap_or("".to_string()),
        ));
    }

    if !response.body.is_empty() {
        if opts.debug {
            file::store_response(response.body.as_ref())?;
        }
        Ok(response)
    } else {
        Err(RusotoError("response body is empty.".to_string()))
    }
}
