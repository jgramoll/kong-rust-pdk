use kong_rust_pdk::server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let args: Vec<String> = std::env::args().collect();
    if args.len() == 1 {
        return Err("Must provide name of plugin".into());
    }
    let plugin_name = &args[1];
    let config = String::from(
        r#"
                {
                    "message": "go on"
                }"#,
    );
    let bind_addr = format!("/usr/local/kong/{}.socket", &plugin_name);
    log::info!("connecting to bind_addr: {}", bind_addr);

    let client =
        server::client::PluginClient::new(plugin_name.to_owned(), config.into_bytes(), bind_addr)
            .await?;

    let ret = client.cmd_start_instance().await?;
    let status = match ret.r#return {
        Some(ret) => match ret {
            pb::rpc_return::Return::PluginNames(_) => todo!(),
            pb::rpc_return::Return::PluginInfo(_) => todo!(),
            pb::rpc_return::Return::InstanceStatus(status) => status,
        },
        None => todo!(),
    };

    // TODO enum for
    let ret = client
        .cmd_handle_event(status.instance_id, String::from("access"))
        .await?;

    log::info!("return from access: {:#?}", ret);

    Ok(())
}
