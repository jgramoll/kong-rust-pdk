use kong_rust_pdk::{macros::*, server, Pdk, Plugin};

const VERSION: &str = "0.1";
const PRIORITY: usize = 1;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    server::start::<Config>(VERSION, PRIORITY).await?;

    Ok(())
}

#[plugin_config]
struct Config {
    message: String,
}

#[plugin_impl]
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
