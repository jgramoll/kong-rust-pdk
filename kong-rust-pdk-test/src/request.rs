use kong_rust_pdk::{async_trait, http::HeaderMap, Error};
use url::Url;

#[derive(Default, Clone)]
pub struct Request {
    pub method: String,
    pub url: String,
    pub headers: HeaderMap,
    pub body: String,
}

impl Request {
    pub fn new(method: &str, url: &str) -> Self {
        Self {
            method: String::from(method),
            url: String::from(url),
            headers: HeaderMap::new(),
            body: String::new(),
        }
    }

    pub(crate) fn validate(&self) -> Result<(), url::ParseError> {
        Url::parse(&self.url)?;
        Ok(())
    }
}

#[async_trait]
impl kong_rust_pdk::request::Request for Request {
    async fn get_scheme(&self) -> Result<String, Error> {
        todo!()
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
        Ok(self.method.clone())
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

    async fn get_query(
        &self,
        _max_args: usize,
    ) -> Result<std::collections::HashMap<String, String>, Error> {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_url() -> Result<(), Box<dyn std::error::Error>> {
        let req = Request::new("GET", "http://example.com/");
        req.validate()?;

        Ok(())
    }

    #[test]
    fn test_validate_bad_url() -> Result<(), Box<dyn std::error::Error>> {
        let req = Request::new("GET", "moo");
        assert!(req.validate().is_err(), "Not a valid url");

        Ok(())
    }
}
