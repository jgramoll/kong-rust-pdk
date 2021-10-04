use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub(crate) enum Methods {
    GetScheme,
    GetMethod,
}

#[derive(Debug, PartialEq)]
pub(crate) struct MethodParseError;

impl FromStr for Methods {
    type Err = MethodParseError;

    fn from_str(s: &str) -> Result<Self, MethodParseError> {
        match s {
            "kong.request.get_scheme" => Ok(Methods::GetScheme),
            "kong.request.get_method" => Ok(Methods::GetMethod),
            _ => Err(MethodParseError),
        }
    }
}

impl std::fmt::Display for Methods {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Methods::GetScheme => "kong.request.get_scheme",
            Methods::GetMethod => "kong.request.get_method",
        };
        write!(f, "{}", str)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_request_methods() {
        assert_eq!("kong.request.get_method", Methods::GetMethod.to_string());
        assert_eq!(
            Methods::GetMethod,
            Methods::from_str("kong.request.get_method").unwrap()
        );
    }

    #[test]
    fn test_request_methods_err() {
        let r = Methods::from_str("foo");
        assert!(r.is_err(), "returns error");
        assert_eq!(MethodParseError, r.err().unwrap());
    }
}
