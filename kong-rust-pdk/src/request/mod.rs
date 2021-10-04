use std::collections::HashMap;

use async_trait::async_trait;
use http::HeaderMap;

use crate::{stream::Stream, Error};

mod methods;

#[async_trait]
pub trait Request: Send + Sync {
    async fn get_scheme(&self) -> Result<String, Error>;
    async fn get_host(&self) -> Result<String, Error>;
    async fn get_port(&self) -> Result<usize, Error>;
    async fn get_forwarded_scheme(&self) -> Result<String, Error>;
    async fn get_forwarded_host(&self) -> Result<String, Error>;
    async fn get_forwarded_port(&self) -> Result<usize, Error>;
    async fn get_http_version(&self) -> Result<f64, Error>;
    async fn get_method(&self) -> Result<String, Error>;
    async fn get_path(&self) -> Result<String, Error>;
    async fn get_path_with_query(&self) -> Result<String, Error>;
    async fn get_raw_query(&self) -> Result<String, Error>;
    async fn get_query_arg(&self, name: String) -> Result<String, Error>;
    async fn get_query(&self, max_args: usize) -> Result<HashMap<String, String>, Error>;
    async fn get_header(&self, name: String) -> Result<String, Error>;
    async fn get_headers(&self, max_headers: usize) -> Result<HeaderMap, Error>;
    async fn get_raw_body(&self) -> Result<String, Error>;
}

#[derive(Debug, Clone)]
pub(crate) struct PbServerRequest {
    stream: Stream,
}

#[async_trait]
impl Request for PbServerRequest {
    async fn get_scheme(&self) -> Result<String, Error> {
        // TODO error type
        self.stream
            .write_method(&methods::Methods::GetScheme.to_string())
            .await
            .unwrap();

        let t = self.stream.read_message::<pb::String>().await.unwrap();
        Ok(t.v)
    }

    async fn get_host(&self) -> Result<String, Error> {
        todo!()
    }

    async fn get_port(&self) -> Result<usize, Error> {
        todo!()
    }

    async fn get_forwarded_scheme(&self) -> Result<String, Error> {
        todo!()
    }

    async fn get_forwarded_host(&self) -> Result<String, Error> {
        todo!()
    }

    async fn get_forwarded_port(&self) -> Result<usize, Error> {
        todo!()
    }

    async fn get_http_version(&self) -> Result<f64, Error> {
        todo!()
    }

    async fn get_method(&self) -> Result<String, Error> {
        // TODO error type
        self.stream
            .write_method(&methods::Methods::GetMethod.to_string())
            .await
            .unwrap();

        let t = self.stream.read_message::<pb::String>().await.unwrap();
        Ok(t.v)
    }

    async fn get_path(&self) -> Result<String, Error> {
        todo!()
    }

    async fn get_path_with_query(&self) -> Result<String, Error> {
        todo!()
    }

    async fn get_raw_query(&self) -> Result<String, Error> {
        todo!()
    }

    async fn get_query_arg(&self, _name: String) -> Result<String, Error> {
        todo!()
    }

    async fn get_query(&self, _max_args: usize) -> Result<HashMap<String, String>, Error> {
        todo!()
    }

    async fn get_header(&self, _name: String) -> Result<String, Error> {
        todo!()
    }

    async fn get_headers(&self, _max_headers: usize) -> Result<HeaderMap, Error> {
        todo!()
    }

    async fn get_raw_body(&self) -> Result<String, Error> {
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
