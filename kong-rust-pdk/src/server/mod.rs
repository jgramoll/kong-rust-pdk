use std::env;

use crate::{server::info::ServerInfoBuilder, Pdk, Plugin};
use pb::{
    self, deserialize_message, rpc_call::Call, rpc_return::Return, serialize_message,
    CmdStartInstance, InstanceStatus, RpcCall, RpcReturn,
};

mod info;

// TODO do we need this to be seperate error?
// struct ServerError;

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
        // What do we do about nullable values
        let _config = match serde_json::from_slice::<T>(&cmd.config) {
            Ok(config) => {
                // WRONG need to wait for access cmd
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
            // TODO config
            config: None,
            started_at: 0,
        };
        Ok(status)
    }
}

// #[tarpc::server]
// #[tonic::async_trait]
// #[tarpc::server]
// Rpc for
impl<T> PluginServer<T>
where
    T: 'static + Plugin,
{
    async fn call(&self, request: RpcCall) -> std::io::Result<RpcReturn> {
        match request.call.unwrap() {
            Call::CmdGetPluginNames(_) => todo!(),
            Call::CmdGetPluginInfo(_) => todo!(),
            Call::CmdStartInstance(cmd) => {
                log::debug!("processing CmdStartInstance");
                // TODO handle unwrap error
                let status = self.start_instance(cmd).unwrap();
                Ok(RpcReturn {
                    sequence: request.sequence,
                    r#return: Some(Return::InstanceStatus(status)),
                })
            }
            Call::CmdGetInstanceStatus(_) => todo!(),
            Call::CmdCloseInstance(_) => todo!(),
            Call::CmdHandleEvent(_) => todo!(),
        }
    }
}

fn get_name() -> String {
    // TODO get name from process
    String::from("example-rust-plugin")
}

fn get_socket_path() -> String {
    // TODO flexible prefix
    let mut path = String::from("/usr/local/kong/");
    path.push_str(&get_name());
    path.push_str(".socket");
    path
}

// TODO can we make a trait / service
async fn handle_client<T>(stream: tokio::net::UnixStream) -> tokio::io::Result<()>
where
    T: 'static + Plugin,
{
    let mut msg = vec![0; 1024];
    let call = loop {
        stream.readable().await?;

        match stream.try_read(&mut msg) {
            Ok(n) => {
                if n == 0 {
                    continue;
                }

                msg.truncate(n);
                break deserialize_message::<RpcCall>(&msg)?;
            }
            Err(ref e) if e.kind() == tokio::io::ErrorKind::WouldBlock => {
                continue;
            }
            Err(e) => {
                return Err(e);
            }
        }
    };

    let server = PluginServer::<T>::new();
    let response = server.call(call).await?;
    let buf = serialize_message(&response);

    loop {
        stream.writable().await?;
        match stream.try_write(&buf) {
            Ok(_n) => {
                break;
            }
            Err(ref e) if e.kind() == tokio::io::ErrorKind::WouldBlock => {
                continue;
            }
            Err(e) => {
                return Err(e);
            }
        }
    }

    Ok(())
}

// TODO map error
pub async fn start<T>(version: &str, priority: usize) -> std::io::Result<()>
where
    T: 'static + Plugin,
{
    // todo args lib
    if env::args().any(|x| x == *"-dump") {
        let dump = ServerInfoBuilder::new::<T>(String::from(version), priority).build();

        // write response to std out
        println!("{}", serde_json::to_string(&dump)?);

        return Ok(());
    }

    // make sure socket doesn't already exist
    let socket_addr = get_socket_path();
    let _ = std::fs::remove_file(&socket_addr);

    let listener = tokio::net::UnixListener::bind(&socket_addr)?;
    log::info!("Listening on socket: {}", &socket_addr);

    loop {
        let (stream, _addr) = listener.accept().await?;
        tokio::spawn(async move { handle_client::<T>(stream).await });
    }
}
