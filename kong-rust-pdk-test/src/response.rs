use std::str::FromStr;

use kong_rust_pdk::{
    async_trait,
    http::{header::HeaderName, HeaderMap, HeaderValue},
    Error,
};

#[derive(Clone)]
pub struct Response {
    pub status: usize,
    pub body: String,
    pub headers: HeaderMap,
}

impl Response {
    pub(crate) fn new() -> Self {
        Self {
            status: 200,
            body: String::new(),
            headers: HeaderMap::default(),
        }
    }
}

#[async_trait]
impl kong_rust_pdk::response::Response for Response {
    async fn get_status(&self) -> std::io::Result<usize> {
        todo!()
    }

    async fn get_header(&self, _name: &str) -> Result<String, kong_rust_pdk::Error> {
        todo!()
    }

    async fn get_headers(
        &self,
        _max_headers: usize,
    ) -> Result<std::iter::Map<String, Vec<String>>, kong_rust_pdk::Error> {
        todo!()
    }

    async fn get_source(&self) -> Result<String, kong_rust_pdk::Error> {
        todo!()
    }

    async fn set_status(&mut self, status: usize) -> Result<(), kong_rust_pdk::Error> {
        self.status = status;
        Ok(())
    }

    async fn set_header(&mut self, name: &str, value: &str) -> Result<(), kong_rust_pdk::Error> {
        // todo wrap error
        let name = HeaderName::from_str(name).unwrap();
        let value = HeaderValue::from_str(value).unwrap();
        self.headers.insert(name, value);
        Ok(())
    }

    async fn add_header(&self, _key: String, _value: String) -> Result<(), kong_rust_pdk::Error> {
        todo!()
    }

    async fn clear_header(&self, _key: String) -> Result<(), kong_rust_pdk::Error> {
        todo!()
    }

    async fn set_headers(
        &self,
        _headers: std::iter::Map<String, Vec<String>>,
    ) -> Result<(), kong_rust_pdk::Error> {
        todo!()
    }

    async fn exit(
        &mut self,
        status: usize,
        body: Option<String>,
        headers: Option<HeaderMap>,
    ) -> Result<(), Error> {
        self.status = status;
        self.body = body.unwrap_or_default();
        self.headers = headers.unwrap_or_default();
        Ok(())
    }

    async fn exit_status(&mut self, status: usize) -> std::io::Result<()> {
        self.status = status;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use kong_rust_pdk::response::Response;

    // use super::*;

    #[tokio::test]
    async fn test_exit_status() {
        let status = 404;
        let mut res = super::Response::new();
        res.exit_status(status).await.unwrap();
        assert_eq!(status, res.status);
    }
}
