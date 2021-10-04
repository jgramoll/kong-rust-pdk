use std::{
    collections::HashMap,
    env,
    sync::{Arc, RwLock},
    time::SystemTime,
};

use tokio::io;

use crate::{pdk::StreamPdk, server::info::ServerInfoBuilder, stream::Stream, Plugin};
use pb::{
    self, rpc_call::Call, rpc_return::Return, CmdCloseInstance, CmdHandleEvent, CmdStartInstance,
    InstanceStatus, RpcCall, RpcReturn,
};

#[cfg(feature = "test_client")]
pub mod client;

mod info;

#[derive(Clone)]
struct Instance<T: Plugin> {
    id: i32,
    start_time: SystemTime,
    config: T,
}

impl<T> Instance<T>
where
    T: Plugin,
{
    fn new(id: i32, config: T) -> Instance<T> {
        Instance::<T> {
            id,
            start_time: SystemTime::now(),
            config,
        }
    }
}

#[derive(Clone)]
pub struct PluginServer<T>
where
    T: Plugin,
{
    instances: Arc<RwLock<HashMap<i32, Instance<T>>>>,
    counter: Arc<RwLock<i32>>,
}

impl<T> PluginServer<T>
where
    T: Plugin,
{
    fn new() -> Self {
        Self {
            instances: Arc::new(RwLock::new(HashMap::new())),
            counter: Arc::new(RwLock::new(0)),
        }
    }

    fn get_instance(&self, instance_id: i32) -> Option<Instance<T>> {
        let instances = self.instances.try_read().unwrap();
        instances.get(&instance_id).cloned()
    }

    fn add_instance(&self, config: T) -> Instance<T> {
        let instance = {
            let mut c = self.counter.try_write().unwrap();
            *c += 1;
            Instance::new(*c, config)
        };

        {
            let mut instances = self.instances.try_write().unwrap();
            instances.insert(instance.id, instance.clone());
        }

        instance
    }

    fn start_instance(&self, cmd: &CmdStartInstance) -> std::io::Result<InstanceStatus> {
        // TODO check name match

        // What do we do about nullable values
        log::debug!(
            " start_instance config {:#?}",
            std::str::from_utf8(&cmd.config)
        );

        let config = serde_json::from_slice::<T>(&cmd.config)?;
        let instance = self.add_instance(config);

        // let t = prost_types::Value::default();
        // let t = serde_json::from_slice::<serde_prost_types::Struct>(&cmd.config)?;
        // let t = serde_prost_types::Value::default();

        Ok(InstanceStatus {
            // name: config.name,
            name: String::default(),
            // name: get_name(),
            instance_id: instance.id,
            // TODO serialize to type
            config: None,
            // config: Some(serde_prost_types::Value {
            //     kind: Some(serde_prost_types::value::Kind::StructValue(
            //         instance.config
            //         // serde_prost_types::from_val::<serde_prost_types::Struct>(&instance.config)
            //         // serde_prost_types::from_val::<serde_prost_types::Struct>(&instance.config)
            //     )),
            // }),
            // config: Some(t),
            started_at: instance
                .start_time
                .duration_since(SystemTime::UNIX_EPOCH)
                // Check error
                .unwrap()
                .as_secs() as i64,
        })
    }

    fn get_instance_status(&self, _id: i32) -> std::io::Result<InstanceStatus> {
        // get instance from hash
        // let instance = Instance::new()

        let status = InstanceStatus {
            name: String::from("---"),
            instance_id: 1,
            // instance_id: instance.id,
            config: None,
            // config: instance.config,
            started_at: 0,
            // started_at: instance
            //     .start_time
            //     .duration_since(SystemTime::UNIX_EPOCH)
            //     // Check error
            //     .unwrap()
            //     .as_secs() as i64,
        };
        Ok(status)
    }

    fn close_instance(&self, cmd: &CmdCloseInstance) -> std::io::Result<InstanceStatus> {
        let _status = self.get_instance_status(cmd.instance_id)?;

        // remove from hash

        todo!();
        // Ok(status)
    }

    async fn handle_event(
        &self,
        stream: Stream,
        cmd: &CmdHandleEvent,
    ) -> Result<(), Box<dyn std::error::Error>> {
        match self.get_instance(cmd.instance_id) {
            Some(instance) => {
                // TODO check if instance has event handler
                log::trace!("Running event {} for instance", cmd.event_name);

                {
                    let mut pdk = StreamPdk::new(stream.clone());

                    // use instance to get config

                    match cmd.event_name.as_str() {
                        "access" => instance.config.access(&mut pdk).await?,
                        _ => todo!(),
                    };
                }

                // trigger empty message to notify we won't send any more calls
                stream.clone().write_frame(&[]).await?;

                Ok(())
            }
            None => panic!("TODO error for missing instance"),
        }
    }

    // todo make a timer to cleanup old instances
    #[allow(dead_code)]
    fn expire_instances() {
        // 60s timeout
        todo!()
    }

    async fn call(&self, stream: Stream, request: RpcCall) -> io::Result<Option<RpcReturn>> {
        let status = match &request.call {
            Some(call) => match call {
                Call::CmdGetPluginNames(_) => None,
                Call::CmdGetPluginInfo(_) => None,
                Call::CmdStartInstance(cmd) => Some(self.start_instance(cmd)?),
                Call::CmdGetInstanceStatus(cmd) => Some(self.get_instance_status(cmd.instance_id)?),
                Call::CmdCloseInstance(cmd) => Some(self.close_instance(cmd)?),
                Call::CmdHandleEvent(cmd) => {
                    self.handle_event(stream, cmd).await.unwrap();

                    return Ok(Some(RpcReturn {
                        sequence: request.sequence,
                        r#return: None,
                    }));
                }
            },
            None => None,
        };

        Ok(status.map(|status| RpcReturn {
            sequence: request.sequence,
            r#return: Some(Return::InstanceStatus(status)),
        }))
    }

    // TODO can we make a trait / service
    async fn handle_rpc_call(&self, stream: Stream) -> tokio::io::Result<()> {
        loop {
            let call = stream.read_message::<RpcCall>().await?;
            let response = self.call(stream.clone(), call).await?;

            if let Some(response) = response {
                let r = stream.write_message(&response).await?;
            } else {
                // let r = stream.write_frame(&[]).await?;
            }
        }
    }
}

pub fn get_name() -> String {
    let name = std::env::args().next().unwrap();
    let name = name.split('/');
    String::from(name.last().unwrap())
}

pub fn get_socket_path() -> String {
    // TODO flexible prefix
    let mut path = String::from("/usr/local/kong/");
    path.push_str(&get_name());
    path.push_str(".socket");
    path
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

    #[cfg(feature = "logger")]
    env_logger::init();

    // make sure socket doesn't already exist
    let socket_addr = get_socket_path();
    let _ = std::fs::remove_file(&socket_addr);

    let listener = tokio::net::UnixListener::bind(&socket_addr)?;
    log::info!("Listening on socket: {}", &socket_addr);

    let server = PluginServer::<T>::new();
    loop {
        log::debug!("Plugin server waiting for connection...");
        let (stream, _addr) = listener.accept().await?;

        let server = server.clone();
        tokio::spawn(async move { server.handle_rpc_call(Stream::new(stream)).await.unwrap() });
    }
}
