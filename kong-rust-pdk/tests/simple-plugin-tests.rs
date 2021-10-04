use kong_rust_pdk::{macros::*, pdk::Pdk, Error, Plugin};

mod common;

const VERSION: &str = "0.1";
const PRIORITY: usize = 1;

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
    async fn access<T: Pdk>(&self, _kong: &mut T) -> Result<(), Error> {
        // TODO
        Ok(())
    }
}

#[test]
#[cfg(feature = "test_client")]
fn test_default_config() -> Result<(), Box<dyn std::error::Error>> {
    use std::time::Instant;

    use kong_rust_pdk::server;
    use tokio::net::UnixStream;

    use crate::common::server_start;

    server_start(Config::default(), VERSION, PRIORITY, async {
        // let plugin_name = String::from("example-rust-plugin");
        let config = String::from(
            r#"
                {
                    "message": "go on"
                }"#,
        );
        let plugin_name = server::get_name();
        let socket_path = server::get_socket_path();

        let now = Instant::now();
        loop {
            if UnixStream::connect(&socket_path).await.is_ok() {
                break;
            }
            if now.elapsed().as_secs() > 1 {
                panic!("Failed to connect");
            }
        }

        let client =
            server::client::PluginClient::new(plugin_name, config.into_bytes(), socket_path)
                .await
                .unwrap();

        let ret = client.cmd_start_instance().await.unwrap();
        let status = match ret.r#return {
            Some(ret) => match ret {
                pb::rpc_return::Return::PluginNames(_) => todo!(),
                pb::rpc_return::Return::PluginInfo(_) => todo!(),
                pb::rpc_return::Return::InstanceStatus(status) => status,
            },
            None => todo!(),
        };

        assert_eq!("", status.name);
        // assert_ne!(None, status.config);
        // let s = serde_json::to_string(&status.config.unwrap());
        // assert_eq!("", s.unwrap());
    })?;

    Ok(())
}
