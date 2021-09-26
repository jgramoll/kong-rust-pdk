use kong_rust_pdk::server;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let plugin_name = String::from("example-rust-plugin");
    let config = String::from(
        r#"
                {
                }"#,
    );
    let bind_addr = "/usr/local/kong/example-rust-plugin.socket";

    let client =
        server::client::PluginClient::new(plugin_name, config.into_bytes(), bind_addr).await?;

    let ret = client.cmd_start_instance().await?;
    println!("{:#?}", ret);

    Ok(())
}
