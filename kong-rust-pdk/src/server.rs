use tarpc::{
    context, serde_transport,
    server::{BaseChannel, Channel},
    tokio_serde::formats::Json,
};
use tokio::net::UnixListener;
use tokio_util::codec::LengthDelimitedCodec;

use crate::{
    kong_plugin_protocol::{
        rpc_call::Call, rpc_return::Return, CmdStartInstance, InstanceStatus, RpcCall, RpcReturn,
    },
    Pdk, Plugin,
};

// TODO do we need this to be seperate error?
struct ServerError;

// RPC service trait
#[tarpc::service]
pub trait Kong {
    async fn rpc_call(message: RpcCall) -> RpcReturn;
}

// TODO does this have to be static?
#[derive(Clone)]
pub struct PluginServer<T>
where
    T: 'static + Plugin,
{
    config: T,
    kong: Pdk,
}

impl<T> PluginServer<T>
where
    T: 'static + Plugin,
{
    fn new() -> Self {
        Self {
            config: T::new(),
            kong: Pdk::new(),
        }
    }
    // fn instance_status() -> Option<Return> {
    //     let status = InstanceStatus {
    //         name: String::from("name"),
    //         instance_id: 1,
    //         config: None,
    //         started_at: 0,
    //     };

    //     Some(Return::InstanceStatus(status))
    // }

    fn start_instance(&self, cmd: CmdStartInstance) -> Result<InstanceStatus, ()> {
        let config = match serde_json::from_slice::<T>(&cmd.config) {
            Ok(config) => {
                println!("got config");

                config.access(&self.kong);

                Some(config)
            }
            Err(e) => {
                // TODO
                println!("error with deserialize config {}", e);
                return Err(());
            }
        };

        // TODO better status
        let status = InstanceStatus {
            name: cmd.name,
            instance_id: 1,
            config: None,
            started_at: 0,
        };
        Ok(status)
    }
}

#[tarpc::server]
impl<T> Kong for PluginServer<T>
where
    T: 'static + Plugin,
{
    async fn rpc_call(self, _: context::Context, message: RpcCall) -> RpcReturn {
        println!("got rpc_call message");
        let c = message.call.unwrap();
        match c {
            Call::CmdGetPluginNames(_) => todo!(),
            Call::CmdGetPluginInfo(_) => todo!(),
            Call::CmdStartInstance(cmd) => {
                let status = match self.start_instance(cmd) {
                    Ok(status) => Some(Return::InstanceStatus(status)),
                    Err(_) => None,
                };
                RpcReturn {
                    sequence: message.sequence,
                    r#return: status,
                }
            }
            Call::CmdGetInstanceStatus(_) => todo!(),
            Call::CmdCloseInstance(_) => todo!(),
            Call::CmdHandleEvent(_) => todo!(),
        }
    }
}

// TODO map error
#[allow(unused_variables)]
pub async fn start<T>(version: &str, priority: usize) -> Result<(), Box<dyn std::error::Error>>
where
    T: 'static + Plugin,
{
    let bind_addr = "/tmp/tarpc_on_unix_example.sock";
    let _ = std::fs::remove_file(bind_addr);

    println!("Starting server...");
    let listener = UnixListener::bind(bind_addr).unwrap();
    let codec_builder = LengthDelimitedCodec::builder();

    loop {
        let (conn, _addr) = listener.accept().await.unwrap();
        let framed = codec_builder.new_framed(conn);
        let transport = serde_transport::new(framed, Json::default());

        let server = PluginServer::<T>::new();
        let fut = BaseChannel::with_defaults(transport).execute(server.serve());
        tokio::spawn(fut);
    }
}
