use request::Request;
use response::Response;
use serde::de::DeserializeOwned;

pub mod bridge;
pub mod client;
pub mod request;
pub mod response;
pub mod server;

pub mod kong_plugin_protocol {
    #![allow(clippy::all)]
    tonic::include_proto!("kong_plugin_protocol");
}

#[derive(Debug, Clone)]
pub struct Error {}

#[derive(Debug, Clone)]
pub struct Pdk {
    pub request: Request,
    pub response: Response,
}

impl Pdk {
    fn new() -> Self {
        Pdk {
            request: Request {},
            response: Response {},
        }
    }
}

// todo Deserialize vs DeserializeOwned
pub trait Plugin: Clone + DeserializeOwned + Send {
    fn new() -> Self;
    fn access(&self, kong: &Pdk);
}
