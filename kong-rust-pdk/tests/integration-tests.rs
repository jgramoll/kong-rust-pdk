// use std::{future::Future, thread, time::Duration};

// use kong_rust_pdk::server;
// use tokio::{net::UnixStream, runtime::Runtime};

// // use crate::start_plugin::start_plugin;

// // use crate::plugin::{Config, PRIORITY, VERSION};

// // fn start_plugin<T>(future: T) -> Result<(), Box<dyn std::error::Error>>
// // where
// //     T: Future + Send + 'static,
// //     T::Output: Send + 'static,
// // {
// //     let rt = Runtime::new().unwrap();
// //     rt.block_on(async {
// //         println!("hello");

// //         tokio::spawn(async {
// //             println!("a");
// //             server::start::<Config>(VERSION, PRIORITY).await.unwrap();
// //             println!("why did this go");
// //         });

// //         // std::thread::sleep(Duration::from_secs(2));

// //         tokio::spawn(future).await?;

// //         println!("futures done");

// //         Ok(())
// //     })
// // }

// // #[test]
// // async fn test_dump() -> Result<(), Box<dyn std::error::Error>> {
// //     server::start(VERSION, PRIORITY)
// // }

// // #[tokio::test]
// #[test]
// #[cfg(feature = "test_client")]
// fn test_start_mock() -> Result<(), Box<dyn std::error::Error>> {
//     start_plugin(async {
//         let plugin_name = String::from("example-rust-plugin");
//         let config = String::from(
//             r#"
//                 {
//                     "message": "go on"
//                 }"#,
//         );
//         let bind_addr = String::from("/usr/local/kong/example-rust-plugin.socket");

//         loop {
//             if UnixStream::connect(&bind_addr).await.is_ok() {
//                 break;
//             }
//         }
//         let client = server::client::PluginClient::new(plugin_name, config.into_bytes(), bind_addr)
//             .await
//             .unwrap();

//         let ret = client.cmd_start_instance().await.unwrap();
//         let status = match ret.r#return {
//             Some(ret) => match ret {
//                 pb::rpc_return::Return::PluginNames(_) => todo!(),
//                 pb::rpc_return::Return::PluginInfo(_) => todo!(),
//                 pb::rpc_return::Return::InstanceStatus(status) => status,
//             },
//             None => todo!(),
//         };

//         // TODO enum for
//         println!("Sending access event");
//         let ret = client
//             .cmd_handle_event(status.instance_id, String::from("access"))
//             .await;
//         // .unwrap();

//         // assert_eq!(ret.sequence, 3);
//         // loop {
//         //     if UnixStream::connect(&bind_addr).await.is_ok() {
//         //         break;
//         //     }
//         // }

//         println!("wow khasldfj {:#?}", ret);
//     })?;

//     assert_eq!(3, 4);

//     Ok(())
// }
