use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub(crate) enum RequestMethods {
    GetScheme,
    GetMethod,
}

#[derive(Debug, PartialEq)]
pub(crate) struct RequestMethodParseError;

impl FromStr for RequestMethods {
    type Err = RequestMethodParseError;

    fn from_str(s: &str) -> Result<Self, RequestMethodParseError> {
        match s {
            "kong.request.get_scheme" => Ok(RequestMethods::GetScheme),
            "kong.request.get_method" => Ok(RequestMethods::GetMethod),
            _ => Err(RequestMethodParseError),
        }
    }
}

impl std::fmt::Display for RequestMethods {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            RequestMethods::GetScheme => "kong.request.get_scheme",
            RequestMethods::GetMethod => "kong.request.get_method",
        };
        write!(f, "{}", str)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_request_methods() {
        assert_eq!(
            "kong.request.get_method",
            RequestMethods::GetMethod.to_string()
        );
        assert_eq!(
            RequestMethods::GetMethod,
            RequestMethods::from_str("kong.request.get_method").unwrap()
        );
    }

    #[test]
    fn test_request_methods_err() {
        let r = RequestMethods::from_str("foo");
        assert!(r.is_err(), "returns error");
        assert_eq!(RequestMethodParseError, r.err().unwrap());
    }
}
