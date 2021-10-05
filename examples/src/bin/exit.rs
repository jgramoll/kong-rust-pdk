use kong_rust_pdk::{macros::*, pdk::Pdk, server, Error, Plugin};

const VERSION: &str = "0.1";
const PRIORITY: usize = 1;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    server::start::<Config>(VERSION, PRIORITY).await?;

    Ok(())
}

#[plugin_config]
struct Config {
    message: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            message: String::from("default message"),
        }
    }
}

#[plugin_impl]
impl Plugin for Config {
    async fn access<T: Pdk>(&self, kong: &mut T) -> Result<(), Error> {
        kong.response()
            .exit(204, Some(String::from("Exit from plugin")), None)
            .await?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use kong_rust_pdk_test::{Request, Test};

    use super::*;

    #[tokio::test]
    async fn test_access() -> Result<(), Box<dyn std::error::Error>> {
        let method = "GET";
        let mut test = Test::new(Request::new(method, "http://example.com?q=search&x=9"))?;

        let res = test.do_https(&Config::default()).await?;

        assert_eq!(204, res.status);
        assert_eq!("Exit from plugin", &res.body);
        Ok(())
    }
}
