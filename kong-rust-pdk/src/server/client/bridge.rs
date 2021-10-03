use tokio::io;

use crate::stream::Stream;

use super::PluginClient;

impl PluginClient {
    pub(crate) async fn handle_method(&self, method: &str, stream: &Stream) -> io::Result<()> {
        println!(" got method {}", method);

        // TODO put these somewhere
        match method {
            "kong.request.get_method" => {
                stream
                    .write_message(&pb::String {
                        v: String::from("GET"),
                    })
                    .await?;
                Ok(())
            }
            _ => {
                println!(" unknown method {}", method);
                Ok(())
            }
        }
    }
}
