// use pb::{kong_client::KongClient, ExitArgs};

// async fn response_exit() -> Result<(), Box<dyn std::error::Error>> {
//     let mut client = KongClient::connect("http://[::1]:50051").await?;

//     let request = tonic::Request::new(ExitArgs::default());
//     let response = client.response_exit(request).await?;

//     println!("RESPONSE={:?}", response);

//     Ok(())
// }
