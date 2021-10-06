use kong_rust_pdk::{Error, Plugin};

pub use log::Log;
pub use request::Request;
pub use response::Response;

mod log;
mod pdk;
mod request;
mod response;

pub struct Test {
    pub pdk: pdk::Pdk,
}

impl Test {
    pub fn new(request: Request) -> Result<Self, Error> {
        request.validate().unwrap();

        Ok(Self {
            pdk: pdk::Pdk::new(request),
        })
    }

    pub async fn do_http<T: Plugin>(&mut self, config: &T) -> Result<&Response, Error> {
        self.do_rewrite(config).await?;
        self.do_access(config).await?;
        self.do_response(config).await?;
        self.do_log(config).await?;

        Ok(&self.pdk.response)
    }

    pub async fn do_https<T: Plugin>(&mut self, config: &T) -> Result<&Response, Error> {
        self.do_certificate(config).await?;
        self.do_http(config).await?;

        Ok(&self.pdk.response)
    }

    pub async fn do_stream<T: Plugin>(&mut self, config: &T) -> Result<(), Error> {
        self.do_preread(config).await?;
        self.do_log(config).await?;
        Ok(())
    }

    pub async fn do_tls<T: Plugin>(&mut self, config: &T) -> Result<(), Error> {
        self.do_certificate(config).await?;
        self.do_stream(config).await?;
        Ok(())
    }

    pub async fn do_certificate<T: Plugin>(&mut self, _config: &T) -> Result<(), Error> {
        Ok(())
    }

    pub async fn do_rewrite<T: Plugin>(&mut self, _config: &T) -> Result<(), Error> {
        Ok(())
    }

    pub async fn do_access<T: Plugin>(&mut self, config: &T) -> Result<(), Error> {
        config.access(&mut self.pdk).await?;
        Ok(())
    }

    pub async fn do_response<T: Plugin>(&mut self, _config: &T) -> Result<(), Error> {
        Ok(())
    }

    pub async fn do_log<T: Plugin>(&mut self, _config: &T) -> Result<(), Error> {
        Ok(())
    }

    pub async fn do_preread<T: Plugin>(&mut self, _config: &T) -> Result<(), Error> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod plugin {
        use kong_rust_pdk::{
            macros::{plugin_config, plugin_impl},
            pdk::Pdk,
            Error, Plugin,
        };

        #[plugin_config]
        #[derive(Default)]
        pub(crate) struct Config {
            message: String,
        }

        #[plugin_impl]
        impl Plugin for Config {
            async fn access<T: Pdk>(&self, _kong: &mut T) -> Result<(), Error> {
                Ok(())
            }
        }
    }

    #[tokio::test]
    async fn test_something() -> Result<(), Box<dyn std::error::Error>> {
        let mut test = Test::new(Request::new("GET", "http://example.com?q=search&x=9"))?;
        let res = test.do_https(&plugin::Config::default()).await?;

        assert_eq!(200, res.status);
        assert_eq!(None, res.headers.get("x-hello-from-rust"));
        Ok(())
    }
}
