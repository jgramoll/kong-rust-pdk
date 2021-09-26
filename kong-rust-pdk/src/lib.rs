use request::Request;
use response::Response;
use serde::{de::DeserializeOwned, Serialize};

pub use macros;
pub use pb;

pub mod bridge;
pub mod client;
pub mod request;
pub mod response;
pub mod server;

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

// TODO trait for each method
// const METHOD_NAMES: [&str; 6] = [
//     "Certificate",
//     "Rewrite",
//     "Access",
//     "Response",
//     "Preread",
//     "Log",
// ];

// TODO Deserialize vs DeserializeOwned
pub trait Plugin:
    Clone + DeserializeOwned + Serialize + Send + Sync + PluginConfig + PluginSchema
{
    fn new() -> Self;
    fn access(&self, kong: &Pdk);
}

pub trait PluginConfig {
    fn get_phases() -> Vec<String>;
}

pub trait PluginSchema {
    fn get_schema() -> String;
}
