use kong_rust_pdk::server;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let plugin_name = String::from("example-rust-plugin");
    let config = String::from(
        r#"
                {
                    "message": "go on"
                }"#,
    );
    let bind_addr = String::from("/usr/local/kong/example-rust-plugin.socket");

    let client =
        server::client::PluginClient::new(plugin_name, config.into_bytes(), bind_addr).await?;

    let ret = client.cmd_start_instance().await?;
    println!("ret {:#?}", ret);

    let status = match ret.r#return {
        Some(ret) => match ret {
            pb::rpc_return::Return::PluginNames(_) => todo!(),
            pb::rpc_return::Return::PluginInfo(_) => todo!(),
            pb::rpc_return::Return::InstanceStatus(status) => status,
        },
        None => todo!(),
    };

    // println!("{:#?}", ret);
    // TODO enum for
    // println!("Sending access event");
    // let ret = client
    //     .cmd_handle_event(status.instance_id, String::from("access"))
    //     .await?;

    // println!("{:#?}", ret);

    Ok(())
}
