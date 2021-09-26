use tokio::{io, net::UnixStream};

use crate::server::socket::{read_from_stream, send_to_stream};
use pb::{rpc_call::Call, CmdStartInstance, RpcCall, RpcReturn};

pub struct PluginClient {
    plugin_name: String,
    plugin_config: Vec<u8>,
    stream: UnixStream,
}

impl PluginClient {
    pub async fn new(
        plugin_name: String,
        plugin_config: Vec<u8>,
        socket_addr: &str,
    ) -> io::Result<PluginClient> {
        let stream = UnixStream::connect(socket_addr).await?;

        Ok(PluginClient {
            plugin_name,
            plugin_config,
            stream,
        })
    }

    pub async fn cmd_start_instance(&self) -> io::Result<RpcReturn> {
        let cmd = RpcCall {
            sequence: 1,
            call: Some(Call::CmdStartInstance(CmdStartInstance {
                name: self.plugin_name.clone(),
                config: self.plugin_config.clone(),
            })),
        };

        send_to_stream(&self.stream, &cmd).await?;
        read_from_stream(&self.stream).await
    }
}
