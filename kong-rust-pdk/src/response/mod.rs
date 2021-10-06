use std::{collections::BTreeMap, io, iter::Map};

use http::HeaderMap;
use strum::{EnumString, IntoStaticStr};

use crate::{async_trait, stream::Stream, Result};

// mod methods;

#[async_trait]
pub trait Response: Send + Sync {
    async fn get_status(&self) -> Result<i32>;
    async fn get_header(&self, name: &str) -> Result<String>;
    async fn get_headers(&self, max_headers: usize) -> Result<Map<String, Vec<String>>>;
    async fn get_source(&self) -> Result<String>;
    async fn set_status(&mut self, status: usize) -> Result<()>;
    async fn set_header(&mut self, key: &str, value: &str) -> Result<()>;
    async fn add_header(&self, key: String, value: String) -> Result<()>;
    async fn clear_header(&self, key: String) -> Result<()>;
    async fn set_headers(&self, headers: Map<String, Vec<String>>) -> Result<()>;
    async fn exit(
        &mut self,
        status: usize,
        body: Option<String>,
        headers: Option<HeaderMap>,
    ) -> Result<()>;
    async fn exit_status(&mut self, status: usize) -> io::Result<()>;
}

#[derive(Debug, PartialEq, IntoStaticStr, EnumString)]
pub(crate) enum Methods {
    #[strum(serialize = "kong.response.get_status")]
    GetStatus,
    #[strum(serialize = "kong.response.get_header")]
    GetHeader,
    #[strum(serialize = "kong.response.get_headers")]
    GetHeaders,
    #[strum(serialize = "kong.response.get_source")]
    GetSource,
    #[strum(serialize = "kong.response.set_status")]
    SetStatus,
    #[strum(serialize = "kong.response.set_header")]
    SetHeader,
    #[strum(serialize = "kong.response.add_header")]
    AddHeader,
    #[strum(serialize = "kong.response.clear_header")]
    ClearHeader,
    #[strum(serialize = "kong.response.set_headers")]
    SetHeaders,
    #[strum(serialize = "kong.response.exit")]
    Exit,
    #[strum(serialize = "kong.response.exit_status")]
    ExitStatus,
}

#[derive(Debug, Clone)]
pub struct PbServerResponse {
    stream: Stream,
}

impl PbServerResponse {
    pub(crate) fn new(stream: Stream) -> Self {
        Self { stream }
    }
}

#[async_trait]
impl Response for PbServerResponse {
    async fn get_status(&self) -> Result<i32> {
        let status = self.stream.ask_int(Methods::GetStatus.into()).await?;
        Ok(status)
    }

    async fn get_header(&self, _name: &str) -> Result<String> {
        todo!()
    }

    async fn get_headers(&self, _max_headers: usize) -> Result<Map<String, Vec<String>>> {
        todo!()
    }

    async fn get_source(&self) -> Result<String> {
        todo!()
    }

    async fn set_status(&mut self, status: usize) -> Result<()> {
        self.stream
            .send_int(Methods::SetStatus.into(), status as i32)
            .await?;
        Ok(())
    }

    async fn set_header(&mut self, name: &str, value: &str) -> Result<()> {
        self.stream
            .ask(
                Methods::SetHeader.into(),
                &pb::Kv::new(String::from(name), Some(String::from(value))),
            )
            .await?;
        Ok(())
    }

    async fn add_header(&self, _key: String, _value: String) -> Result<()> {
        todo!()
    }

    async fn clear_header(&self, _key: String) -> Result<()> {
        todo!()
    }

    async fn set_headers(&self, _headers: Map<String, Vec<String>>) -> Result<()> {
        todo!()
    }

    async fn exit(
        &mut self,
        status: usize,
        body: Option<String>,
        _headers: Option<HeaderMap>,
    ) -> Result<()> {
        self.stream
            .ask(
                Methods::Exit.into(),
                &pb::ExitArgs {
                    status: status as i32,
                    body: body.unwrap_or_default(),
                    headers: Some(prost_types::Struct {
                        fields: BTreeMap::new(),
                    }),
                },
            )
            .await?;

        Ok(())
    }

    async fn exit_status(&mut self, _status: usize) -> io::Result<()> {
        todo!()
    }
}
