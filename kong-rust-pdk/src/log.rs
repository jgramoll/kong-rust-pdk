use strum::{EnumString, IntoStaticStr};

use crate::{async_trait, stream::Stream, Result};

#[async_trait]
pub trait Log: Send + Sync {
    async fn alert(&mut self, args: String) -> Result<()>;
    async fn crit(&self, args: String) -> Result<()>;
    async fn err(&self, args: String) -> Result<()>;
    async fn warn(&self, args: String) -> Result<()>;
    async fn notice(&self, args: String) -> Result<()>;
    async fn info(&self, args: String) -> Result<()>;
    async fn debug(&self, args: String) -> Result<()>;
    async fn serialize(&self) -> Result<String>;
}

#[derive(Debug, PartialEq, IntoStaticStr, EnumString)]
pub(crate) enum Methods {
    #[strum(serialize = "kong.log.alert")]
    Alert,
    #[strum(serialize = "kong.log.crit")]
    Crit,
    #[strum(serialize = "kong.log.err")]
    Error,
    #[strum(serialize = "kong.log.warn")]
    Warn,
    #[strum(serialize = "kong.log.notice")]
    Notice,
    #[strum(serialize = "kong.log.info")]
    Info,
    #[strum(serialize = "kong.log.debug")]
    Debug,
    #[strum(serialize = "kong.log.serialize")]
    Serialize,
}

#[derive(Clone)]
pub(crate) struct PbLog {
    stream: Stream,
}

impl PbLog {
    pub(crate) fn new(stream: Stream) -> Self {
        Self { stream }
    }

    // TODO can we pass more than 1 arg
    async fn do_log(&self, method: Methods, args: String) -> Result<()> {
        let value = serde_prost_types::Value {
            kind: Some(serde_prost_types::value::Kind::StringValue(args)),
        };
        let args = serde_prost_types::ListValue {
            values: vec![value],
        };
        self.stream.ask(method.into(), &args).await
    }
}

#[async_trait]
impl Log for PbLog {
    async fn alert(&mut self, args: String) -> Result<()> {
        self.do_log(Methods::Alert, args).await
    }
    async fn crit(&self, args: String) -> Result<()> {
        self.do_log(Methods::Crit, args).await
    }
    async fn err(&self, args: String) -> Result<()> {
        self.do_log(Methods::Error, args).await
    }
    async fn warn(&self, args: String) -> Result<()> {
        self.do_log(Methods::Warn, args).await
    }
    async fn notice(&self, args: String) -> Result<()> {
        self.do_log(Methods::Notice, args).await
    }
    async fn info(&self, args: String) -> Result<()> {
        self.do_log(Methods::Info, args).await
    }
    async fn debug(&self, args: String) -> Result<()> {
        self.do_log(Methods::Debug, args).await
    }
    async fn serialize(&self) -> Result<String> {
        self.stream.ask_string(Methods::Serialize.into()).await
    }
}
