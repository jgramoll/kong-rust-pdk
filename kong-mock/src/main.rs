use tarpc::serde_transport;
use tarpc::{context, tokio_serde::formats::Json};
use tokio::net::UnixStream;
use tokio_util::codec::LengthDelimitedCodec;

use kong_rust_pdk::kong_plugin_protocol::{rpc_call::Call, CmdStartInstance, RpcCall};
use kong_rust_pdk::server::KongClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let bind_addr = "/tmp/tarpc_on_unix_example.sock";
    let conn = UnixStream::connect(bind_addr).await?;

    let codec_builder = LengthDelimitedCodec::builder();
    let transport = serde_transport::new(codec_builder.new_framed(conn), Json::default());

    println!("opening client");
    let client = KongClient::new(Default::default(), transport).spawn();

    let config = String::from(
        r#"
        {
            "message": "In a bottle"
        }"#,
    );

    let cmd = CmdStartInstance {
        name: String::from("plugin_name"),
        config: config.into_bytes(),
    };
    let call = RpcCall {
        sequence: 1,
        call: Some(Call::CmdStartInstance(cmd)),
    };
    let r = client.rpc_call(context::current(), call).await?;

    println!("resp {:?}", r);

    Ok(())
}
