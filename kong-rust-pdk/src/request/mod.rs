use std::collections::HashMap;

use http::HeaderMap;
use strum::{EnumString, IntoStaticStr};

use crate::{async_trait, stream::Stream, Result};

#[async_trait]
pub trait Request: Send + Sync {
    async fn get_scheme(&self) -> Result<String>;
    async fn get_host(&self) -> Result<String>;
    async fn get_port(&self) -> Result<usize>;
    async fn get_forwarded_scheme(&self) -> Result<String>;
    async fn get_forwarded_host(&self) -> Result<String>;
    async fn get_forwarded_port(&self) -> Result<usize>;
    async fn get_http_version(&self) -> Result<f64>;
    async fn get_method(&self) -> Result<String>;
    async fn get_path(&self) -> Result<String>;
    async fn get_path_with_query(&self) -> Result<String>;
    async fn get_raw_query(&self) -> Result<String>;
    async fn get_query_arg(&self, name: String) -> Result<String>;
    async fn get_query(&self, max_args: usize) -> Result<HashMap<String, String>>;
    async fn get_header(&self, name: String) -> Result<String>;
    async fn get_headers(&self, max_headers: usize) -> Result<HeaderMap>;
    async fn get_raw_body(&self) -> Result<String>;
}

#[derive(Debug, PartialEq, IntoStaticStr, EnumString)]
pub(crate) enum Methods {
    #[strum(serialize = "kong.request.get_scheme")]
    GetScheme,
    #[strum(serialize = "kong.request.get_method")]
    GetMethod,
}

#[derive(Debug, Clone)]
pub(crate) struct PbServerRequest {
    stream: Stream,
}

#[async_trait]
impl Request for PbServerRequest {
    async fn get_scheme(&self) -> Result<String> {
        self.stream.ask_string(Methods::GetScheme.into()).await
    }

    async fn get_host(&self) -> Result<String> {
        todo!()
    }

    async fn get_port(&self) -> Result<usize> {
        todo!()
    }

    async fn get_forwarded_scheme(&self) -> Result<String> {
        todo!()
    }

    async fn get_forwarded_host(&self) -> Result<String> {
        todo!()
    }

    async fn get_forwarded_port(&self) -> Result<usize> {
        todo!()
    }

    async fn get_http_version(&self) -> Result<f64> {
        todo!()
    }

    async fn get_method(&self) -> Result<String> {
        // TODO error type
        self.stream
            .write_method(Methods::GetMethod.into())
            .await
            .unwrap();

        let t = self.stream.read_message::<pb::String>().await.unwrap();
        Ok(t.v)
    }

    async fn get_path(&self) -> Result<String> {
        todo!()
    }

    async fn get_path_with_query(&self) -> Result<String> {
        todo!()
    }

    async fn get_raw_query(&self) -> Result<String> {
        todo!()
    }

    async fn get_query_arg(&self, _name: String) -> Result<String> {
        todo!()
    }

    async fn get_query(&self, _max_args: usize) -> Result<HashMap<String, String>> {
        todo!()
    }

    async fn get_header(&self, _name: String) -> Result<String> {
        todo!()
    }

    async fn get_headers(&self, _max_headers: usize) -> Result<HeaderMap> {
        todo!()
    }

    async fn get_raw_body(&self) -> Result<String> {
        todo!()
    }
}

impl PbServerRequest {
    pub(crate) fn new(stream: Stream) -> Self {
        Self { stream }
    }
}

#[cfg(test)]
mod tests {
    use crate::stream::tests::new_stream;
    use core::result::Result;

    use super::*;

    #[tokio::test]
    async fn test_get_scheme() -> Result<(), Box<dyn std::error::Error>> {
        let expected_scheme = String::from("http");

        let (left, right) = new_stream()?;

        right
            .write_message(&pb::String {
                v: expected_scheme.clone(),
            })
            .await?;

        let r = PbServerRequest::new(left);
        let method = r.get_scheme().await?;
        assert_eq!(expected_scheme, method);

        // TODO not working
        // let s = right.read_method().await?;
        // assert_eq!(RequestMethods::GetScheme.to_string(), s);

        Ok(())
    }

    #[tokio::test]
    async fn test_get_method() -> Result<(), Box<dyn std::error::Error>> {
        let expected_method = String::from("GET");

        let (left, right) = new_stream()?;

        right
            .write_message(&pb::String {
                v: expected_method.clone(),
            })
            .await?;

        let r = PbServerRequest::new(left);
        let method = r.get_method().await.unwrap();
        assert_eq!(expected_method, method);

        let s = right.read_method().await?;
        assert_eq!(methods::Methods::GetMethod.to_string(), s);

        Ok(())
    }
}
