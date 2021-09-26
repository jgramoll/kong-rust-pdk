// use crate::kong_plugin_protocol::kong_client::KongClient;
use crate::Error;

#[derive(Debug, Clone)]
pub struct Request {
    // bridge: Bridge
// client: KongClient,
}

impl Request {
    pub fn get_header(&self, _name: &str) -> Result<String, Error> {
        // let client = KongClient;
        // KongClient::request_get_header(&mutself, request)
        // let header = KongClient::request_get_header(&mutself, request);

        let s = String::from("foo");
        // Err(Error {})
        Ok(s)
    }
}
