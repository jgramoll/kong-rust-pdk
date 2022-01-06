use std::{collections::BTreeMap, iter::Map};

use http::HeaderMap;

use super::{Methods, Response};
use crate::{async_trait, stream::Stream, Result};

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
        self.stream.ask_int(Methods::GetStatus.into()).await
    }

    async fn get_header(&self, name: &str) -> Result<String> {
        self.stream
            .ask_string_with_args(
                Methods::GetHeader.into(),
                &pb::String {
                    v: String::from(name),
                },
            )
            .await
    }

    async fn get_headers(&self, max_headers: usize) -> Result<HeaderMap> {
        let headers: serde_prost_types::Struct = self
            .stream
            .ask_message(
                Methods::GetHeaders.into(),
                &pb::Int {
                    v: max_headers as i32,
                },
            )
            .await?;

        Stream::unwrap_headers(headers)
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

    async fn exit_status(&mut self, _status: usize) -> Result<()> {
        todo!()
    }
}
