use std::io::Cursor;

use tokio::{io, net::UnixStream};

use kong_rust_pdk::pb::{rpc_call::Call, CmdStartInstance, RpcCall, RpcReturn};

use prost::Message;

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

fn serialize_cmd_start_instance(call: &RpcCall) -> Vec<u8> {
    let mut buf = Vec::new();
    buf.reserve(call.encoded_len());
    call.encode_length_delimited(&mut buf).unwrap();
    buf
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
    let buf = serialize_cmd_start_instance(&cmd);

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

    println!("msg {:?}", &msg);
    println!("msg {:?}", std::str::from_utf8(&msg));
    let ret = RpcReturn::decode_length_delimited(Cursor::new(&msg))?;
    Ok(ret)
}
