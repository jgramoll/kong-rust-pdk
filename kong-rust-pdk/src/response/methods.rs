use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub(crate) enum Methods {
    GetStatus,
}

#[derive(Debug, PartialEq)]
pub(crate) struct MethodParseError;

impl FromStr for Methods {
    type Err = MethodParseError;

    fn from_str(s: &str) -> Result<Self, MethodParseError> {
        match s {
            "kong.response.get_status" => Ok(Methods::GetStatus),
            _ => Err(MethodParseError),
        }
    }
}

impl std::fmt::Display for Methods {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Methods::GetStatus => "kong.response.get_status",
        };
        write!(f, "{}", str)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_string() {
        assert_eq!("kong.response.get_status", Methods::GetStatus.to_string());
    }

    #[test]
    fn test_from_string() {
        assert_eq!(
            Methods::GetStatus,
            Methods::from_str("kong.response.get_status").unwrap()
        );
    }

    #[test]
    fn test_request_methods_err() {
        let r = Methods::from_str("foo");
        assert!(r.is_err(), "returns error");
        assert_eq!(MethodParseError, r.err().unwrap());
    }
}
