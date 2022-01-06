use std::collections::HashMap;

use http::HeaderMap;
use strum::{Display, EnumString, IntoStaticStr};

use crate::{async_trait, stream::Stream, Result};

#[async_trait]
pub trait Request: Send + Sync {
    // kong.Request.GetScheme() returns the scheme component of the request’s URL.
    // The returned value is normalized to lower-case form.
    async fn get_scheme(&self) -> Result<String>;

    // kong.Request.GetHost() returns the host component of the request’s URL,
    // or the value of the “Host” header. The returned value is normalized
    // to lower-case form.
    async fn get_host(&self) -> Result<String>;

    // kong.Request.GetPort() returns the port component of the request’s URL.
    async fn get_port(&self) -> Result<usize>;

    // kong.Request.GetForwardedScheme() returns the scheme component
    // of the request’s URL, but also considers X-Forwarded-Proto if it
    // comes from a trusted source. The returned value is normalized to lower-case.
    //
    // Whether this function considers X-Forwarded-Proto or not depends
    // on several Kong configuration parameters:
    //
    //   - trusted_ips
    //   - real_ip_header
    //   - real_ip_recursive
    //
    // Note: support for the Forwarded HTTP Extension (RFC 7239) is not offered yet
    // since it is not supported by ngx_http_realip_module.
    async fn get_forwarded_scheme(&self) -> Result<String>;

    // kong.Request.GetForwardedHost() returns the host component of the request’s URL
    // or the value of the “host” header. Unlike kong.Request.GetHost(), this function
    // will also consider X-Forwarded-Host if it comes from a trusted source.
    // The returned value is normalized to lower-case.
    //
    // Whether this function considers X-Forwarded-Proto or not depends
    // on several Kong configuration parameters:
    //
    //   - trusted_ips
    //   - real_ip_header
    //   - real_ip_recursive
    //
    // Note: we do not currently offer support for Forwarded HTTP Extension (RFC 7239)
    // since it is not supported by ngx_http_realip_module.
    async fn get_forwarded_host(&self) -> Result<String>;

    // kong.Request.GetForwardedPort() returns the port component of the request’s URL,
    // but also considers X-Forwarded-Host if it comes from a trusted source.
    //
    // Whether this function considers X-Forwarded-Proto or not depends
    // on several Kong configuration parameters:
    //
    //   - trusted_ips
    //   - real_ip_header
    //   - real_ip_recursive
    //
    // Note: we do not currently offer support for Forwarded HTTP Extension (RFC 7239)
    // since it is not supported by ngx_http_realip_module.
    async fn get_forwarded_port(&self) -> Result<usize>;

    // kong.Request.GetHttpVersion() returns the HTTP version
    // used by the client in the request, returning values
    // such as "1"", "1.1", "2.0", or nil for unrecognized values.
    async fn get_http_version(&self) -> Result<f64>;

    // kong.Request.GetMethod() returns the HTTP method of the request.
    // The value is normalized to upper-case.
    async fn get_method(&self) -> Result<String>;

    // kong.Request.GetPath() returns the path component of the request’s URL.
    // It is not normalized in any way and does not include the querystring.
    async fn get_path(&self) -> Result<String>;

    // kong.Request.GetPathWithQuery() returns the path, including
    // the querystring if any. No transformations/normalizations are done.
    async fn get_path_with_query(&self) -> Result<String>;

    // kong.Request.GetRawQuery() returns the query component of the request’s URL.
    // It is not normalized in any way (not even URL-decoding of special characters)
    // and does not include the leading ? character.
    async fn get_raw_query(&self) -> Result<String>;

    // kong.Request.GetQueryArg() returns the value of the specified argument,
    // obtained from the query arguments of the current request.
    //
    // The returned value is either a string, a boolean true if
    // an argument was not given a value, or nil if no argument with name was found.
    //
    // If an argument with the same name is present multiple times in the querystring,
    // this function will return the value of the first occurrence.
    async fn get_query_arg(&self, name: String) -> Result<String>;

    // kong.Request.GetQuery() returns a map of query arguments
    // obtained from the querystring. Keys are query argument names.
    // Values are either a string with the argument value, a boolean true
    // if an argument was not given a value, or an array if an argument
    // was given in the query string multiple times. Keys and values are
    // unescaped according to URL-encoded escaping rules.
    //
    // Note that a query string `?foo&bar` translates to two boolean true arguments,
    // and ?foo=&bar= translates to two string arguments containing empty strings.
    //
    // The max_args argument specifies the maximum number of returned arguments.
    // Must be greater than 1 and not greater than 1000, or -1 to specify the
    // default limit of 100 arguments.
    async fn get_query(&self, max_args: usize) -> Result<HashMap<String, String>>;

    // kong.Request.GetHeader() returns the value of the specified request header.
    //
    // The returned value is either a string, or can be nil if a header with name
    // was not found in the request. If a header with the same name is present
    // multiple times in the request, this function will return the value of the
    // first occurrence of this header.
    //
    // Header names in are case-insensitive and are normalized to lowercase,
    // and dashes (-) can be written as underscores (_); that is, the header
    // X-Custom-Header can also be retrieved as x_custom_header.
    async fn get_header(&self, name: String) -> Result<String>;

    // kong.Request.GetHeaders() returns a map holding the request headers.
    // Keys are header names. Values are either a string with the header value,
    // or an array of strings if a header was sent multiple times. Header names
    // in this table are case-insensitive and are normalized to lowercase,
    // and dashes (-) can be written as underscores (_); that is, the header
    // X-Custom-Header can also be retrieved as x_custom_header.
    //
    // The max_args argument specifies the maximum number of returned headers.
    // Must be greater than 1 and not greater than 1000, or -1 to specify the
    // default limit of 100 headers.
    async fn get_headers(&self, max_headers: usize) -> Result<HeaderMap>;

    // kong.request().get_raw_body() returns the plain request body.
    //
    // If the body has no size (empty), this function returns an empty string.
    //
    // If the size of the body is greater than the Nginx buffer size
    // (set by client_body_buffer_size), this function will fail
    // and return an error message explaining this limitation.
    async fn get_raw_body(&self) -> Result<String>;
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug, PartialEq, IntoStaticStr, EnumString, Display)]
pub(crate) enum Methods {
    #[strum(serialize = "kong.request.get_scheme")]
    GetScheme,
    #[strum(serialize = "kong.request.get_host")]
    GetHost,
    #[strum(serialize = "kong.request.get_port")]
    GetPort,
    #[strum(serialize = "kong.request.get_forwarded_scheme")]
    GetForwardedScheme,
    #[strum(serialize = "kong.request.get_forwarded_host")]
    GetForwardedHost,
    #[strum(serialize = "kong.request.get_forwarded_port")]
    GetForwardedPort,
    #[strum(serialize = "kong.request.get_http_version")]
    GetHttpVersion,
    #[strum(serialize = "kong.request.get_method")]
    GetMethod,
    #[strum(serialize = "kong.request.get_path")]
    GetPath,
    #[strum(serialize = "kong.request.get_path_with_query")]
    GetPathWithQuery,
    #[strum(serialize = "kong.request.get_raw_query")]
    GetRawQuery,
    #[strum(serialize = "kong.request.get_query_arg")]
    GetQueryArg,
    #[strum(serialize = "kong.request.get_query")]
    GetQuery,
    #[strum(serialize = "kong.request.get_header")]
    GetHeader,
    #[strum(serialize = "kong.request.get_headers")]
    GetHeaders,
    #[strum(serialize = "kong.request.get_raw_body")]
    GetRawBody,
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
        self.stream.ask_string(Methods::GetHost.into()).await
    }

    async fn get_port(&self) -> Result<usize> {
        self.stream
            .ask_int(Methods::GetPort.into())
            .await
            .map(|port| port as usize)
    }

    async fn get_forwarded_scheme(&self) -> Result<String> {
        self.stream
            .ask_string(Methods::GetForwardedScheme.into())
            .await
    }

    async fn get_forwarded_host(&self) -> Result<String> {
        self.stream
            .ask_string(Methods::GetForwardedHost.into())
            .await
    }

    async fn get_forwarded_port(&self) -> Result<usize> {
        self.stream
            .ask_int(Methods::GetForwardedPort.into())
            .await
            .map(|port| port as usize)
    }

    async fn get_http_version(&self) -> Result<f64> {
        self.stream
            .ask_number(Methods::GetForwardedPort.into())
            .await
    }

    async fn get_method(&self) -> Result<String> {
        self.stream.ask_string(Methods::GetMethod.into()).await
    }

    async fn get_path(&self) -> Result<String> {
        self.stream.ask_string(Methods::GetPath.into()).await
    }

    async fn get_path_with_query(&self) -> Result<String> {
        self.stream
            .ask_string(Methods::GetPathWithQuery.into())
            .await
    }

    async fn get_raw_query(&self) -> Result<String> {
        self.stream.ask_string(Methods::GetRawQuery.into()).await
    }

    async fn get_query_arg(&self, name: String) -> Result<String> {
        self.stream
            .ask_string_with_args(Methods::GetQueryArg.into(), &pb::String { v: name })
            .await
    }

    async fn get_query(&self, _max_args: usize) -> Result<HashMap<String, String>> {
        // if max_args == -1 {
        // 	max_args = 100
        // }

        // arg := kong_plugin_protocol.Int{V: int32(max_args)}
        // out := new(structpb.Struct)
        // err := r.Ask("kong.request.get_query", &arg, out)
        // if err != nil {
        // 	return nil, err
        // }

        // return bridge.UnwrapHeaders(out), nil
        todo!()
    }

    async fn get_header(&self, _name: String) -> Result<String> {
        self.stream.ask_string(Methods::GetHeader.into()).await
    }

    async fn get_headers(&self, _max_headers: usize) -> Result<HeaderMap> {
        // if max_headers == -1 {
        // 	max_headers = 100
        // }

        // arg := kong_plugin_protocol.Int{V: int32(max_headers)}
        // out := new(structpb.Struct)
        // err := r.Ask("kong.request.get_headers", &arg, out)
        // if err != nil {
        // 	return nil, err
        // }

        // return bridge.UnwrapHeaders(out), nil
        todo!()
    }

    async fn get_raw_body(&self) -> Result<String> {
        self.stream.ask_string(Methods::GetHeader.into()).await
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

        let s = right.read_method().await?;
        assert_eq!(Methods::GetScheme.to_string(), s);

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
        assert_eq!(Methods::GetMethod.to_string(), s);

        Ok(())
    }

    #[tokio::test]
    async fn test_get_port() -> Result<(), Box<dyn std::error::Error>> {
        let expected_port: usize = 1000;

        let (left, right) = new_stream()?;

        right
            .write_message(&pb::Int {
                v: expected_port as i32,
            })
            .await?;

        let r = PbServerRequest::new(left);
        let port = r.get_port().await.unwrap();
        assert_eq!(expected_port, port);

        let s = right.read_method().await?;
        assert_eq!(Methods::GetPort.to_string(), s);

        Ok(())
    }

    #[tokio::test]
    async fn test_get_forwarded_scheme() -> Result<(), Box<dyn std::error::Error>> {
        let expected_scheme = String::from("HTTP");

        let (left, right) = new_stream()?;

        right
            .write_message(&pb::String {
                v: expected_scheme.clone(),
            })
            .await?;

        let r = PbServerRequest::new(left);
        let scheme = r.get_forwarded_scheme().await.unwrap();
        assert_eq!(expected_scheme, scheme);

        let s = right.read_method().await?;
        assert_eq!(Methods::GetForwardedScheme.to_string(), s);

        Ok(())
    }

    #[tokio::test]
    async fn test_get_forwarded_host() -> Result<(), Box<dyn std::error::Error>> {
        let expected_host = String::from("forwarded_host");

        let (left, right) = new_stream()?;

        right
            .write_message(&pb::String {
                v: expected_host.clone(),
            })
            .await?;

        let r = PbServerRequest::new(left);
        let host = r.get_forwarded_host().await.unwrap();
        assert_eq!(expected_host, host);

        let s = right.read_method().await?;
        assert_eq!(Methods::GetForwardedHost.to_string(), s);

        Ok(())
    }
    // TODO more tests
}
