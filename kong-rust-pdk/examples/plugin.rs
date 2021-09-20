use serde::Deserialize;

use kong_rust_pdk::{server, Pdk, Plugin};

const VERSION: &str = "0.1";
const PRIORITY: usize = 1;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    server::start::<Config>(VERSION, PRIORITY).await?;

    Ok(())
}

#[derive(Default, Deserialize, Clone)]
struct Config {
    message: String,
}

impl Plugin for Config {
    fn new() -> Config {
        Config::default()
    }

    fn access(&self, kong: &Pdk) {
        let host = kong
            .request
            .get_header("host")
            .expect("Error reading 'host' header");

        println!("Here in plugin {} {}", host, self.message);

        let message = &self.message;
        kong.response
            .set_header(
                "x-hello-from-rust",
                &format!("Rust says {} to {}", message, host),
            )
            .expect("Error setting header");
    }
}
