use prost::Message;
use tokio::{io, net::UnixStream};

use crate::stream::Stream;
use pb::{rpc_call::Call, CmdHandleEvent, CmdStartInstance, RpcCall, RpcReturn};

pub(crate) mod bridge;

pub struct PluginClient {
    plugin_name: String,
    plugin_config: Vec<u8>,
    stream: Stream,
}

impl PluginClient {
    pub async fn new(
        plugin_name: String,
        plugin_config: Vec<u8>,
        socket_addr: String,
    ) -> io::Result<PluginClient> {
        Ok(PluginClient {
            plugin_name,
            plugin_config,
            stream: Stream::new(UnixStream::connect(socket_addr).await?),
        })
    }

    // async fn get_stream(&self) -> io::Result<Stream> {
    //     Ok()
    // }

    pub async fn cmd_start_instance(&self) -> io::Result<RpcReturn> {
        let cmd = RpcCall {
            sequence: 1,
            call: Some(Call::CmdStartInstance(CmdStartInstance {
                name: self.plugin_name.clone(),
                config: self.plugin_config.clone(),
            })),
        };

        self.stream.write_message(&cmd).await?;
        self.stream.read_message().await
    }

    pub async fn cmd_handle_event(
        &self,
        instance_id: i32,
        event_name: String,
    ) -> Result<RpcReturn, Box<dyn std::error::Error>> {
        let cmd = RpcCall {
            sequence: 1,
            call: Some(Call::CmdHandleEvent(CmdHandleEvent {
                instance_id,
                event_name,
            })),
        };

        self.stream.write_message(&cmd).await?;

        // handle methods until we get empty method which triggers end of call
        loop {
            let method = self.stream.read_method().await?;
            if method.is_empty() {
                break;
            }
            self.handle_method(&method, &self.stream).await?
        }

        let bytes = self.stream.read_frame().await?;
        let ret = pb::RpcReturn::decode(&*bytes)?;
        Ok(ret)
    }
}
