use std::fmt::{self, Debug};

use pdk::Pdk;
use serde::{de::DeserializeOwned, Serialize};

pub use async_trait::async_trait;
pub use http;
pub use macros;
pub use pb;

pub mod pdk;
pub mod request;
pub mod response;
pub mod server;

pub(crate) mod stream;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone)]
pub struct Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "SuperError is here!")
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }

    // fn cause(&self) -> Option<&dyn std::error::Error> {
    //     self.source()
    // }
}

impl From<std::io::Error> for Error {
    fn from(_: std::io::Error) -> Self {
        todo!()
    }
}

// enum MethodNames {
//     Certificate,
//     Rewrite,
//     Access,
//     Response,
//     Preread,
//     Log,
// }

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

#[async_trait]
pub trait Plugin:
    // TODO organize better
    Clone + DeserializeOwned + Default + PluginConfig + PluginSchema + Send + Serialize + Sync
{
    async fn access<T: Pdk>(&self, kong: &mut T) -> Result<()>;
}

pub trait PluginConfig {
    fn get_phases() -> Vec<String>;
}

pub trait PluginSchema {
    fn get_schema() -> String;
}
