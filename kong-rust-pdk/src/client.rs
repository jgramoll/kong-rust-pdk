use crate::kong_plugin_protocol::kong_client::KongClient;
use crate::kong_plugin_protocol::ExitArgs;

async fn response_exit() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = KongClient::connect("http://[::1]:50051").await?;

    let request = tonic::Request::new(ExitArgs::default());
    let response = client.response_exit(request).await?;

    println!("RESPONSE={:?}", response);

    Ok(())
}
