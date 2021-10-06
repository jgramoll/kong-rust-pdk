use crate::stream::Stream;

use super::PluginClient;

impl PluginClient {
    pub(crate) async fn handle_method(
        &self,
        method: &str,
        stream: &Stream,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // TODO put these somewhere
        match method {
            // RequestMethods::GetMethod.to
            "kong.request.get_scheme" => {
                stream
                    .write_message(&pb::String {
                        v: String::from("HTTP"),
                    })
                    .await?;
                Ok(())
            }
            "kong.request.get_method" => {
                stream
                    .write_message(&pb::String {
                        v: String::from("GET"),
                    })
                    .await?;
                Ok(())
            }
            "kong.response.exit" => Ok(()),
            _ => Err(format!(" unknown method {}", method).into()),
        }
    }
}
