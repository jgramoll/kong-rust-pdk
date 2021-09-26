use tokio::{io, net::UnixStream};

use kong_rust_pdk::pb::{
    deserialize_message, rpc_call::Call, serialize_message, CmdStartInstance, RpcCall, RpcReturn,
};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let bind_addr = "/usr/local/kong/example-rust-plugin.socket";
    // let bind_addr = "/usr/local/kong/go-log.socket";

    let stream = UnixStream::connect(bind_addr).await?;

    send_start(&stream).await?;
    let ret = read_result(&stream).await?;
    println!("ret {:?}", &ret);

    Ok(())
}

fn create_cmd_start_instance(plugin_name: std::string::String, config: Vec<u8>) -> RpcCall {
    RpcCall {
        sequence: 1,
        call: Some(Call::CmdStartInstance(CmdStartInstance {
            name: plugin_name,
            config,
        })),
    }
}

// todo rpc stream service?
async fn send_start(stream: &UnixStream) -> tokio::io::Result<()> {
    let plugin_name = String::from("example-rust-plugin");
    let config = String::from(
        r#"
            {
                "message": "In a bottle"
            }"#,
    );
    let cmd = create_cmd_start_instance(plugin_name, config.into_bytes());
    let buf = serialize_message(&cmd);

    send(stream, &buf).await
}

async fn send(stream: &UnixStream, buf: &[u8]) -> tokio::io::Result<()> {
    loop {
        stream.writable().await?;

        match stream.try_write(buf) {
            Ok(_n) => {
                break;
            }
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                continue;
            }
            Err(e) => {
                return Err(e);
            }
        }
    }
    Ok(())
}

async fn read_result(stream: &UnixStream) -> tokio::io::Result<RpcReturn> {
    let mut msg = vec![0; 1024];

    loop {
        stream.readable().await?;

        match stream.try_read(&mut msg) {
            Ok(n) => {
                msg.truncate(n);
                break;
            }
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                continue;
            }
            Err(e) => {
                return Err(e);
            }
        }
    }

    let ret = deserialize_message(&msg)?;
    Ok(ret)
}
