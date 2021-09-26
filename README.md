# Kong Rust PDK

## Example

```toml
# Cargo.toml
[dependencies]
kong-rust-pdk = { git = "https://github.com/jgramoll/kong-rust-pdk" }
serde = "1.0"
tokio = { version = "1.11", features = ["macros", "rt-multi-thread"] }
```

```rs
// src/main.rs
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
    message: Option<String>,
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

        let message = &self.message;
        kong.response
            .set_header(
                "x-hello-from-rust",
                &format!("Rust says {:?} to {}", message, host),
            )
            .expect("Error setting header");
    }
}
```

## Modules

* kong-rust-pdk-macro - macros for use in pdk implementation
* kong-rust-pdk-proto - convert .proto file to rust objects
* kong-rust-pdk       - core logic for pluginserver
* serde-prost-types   - wrapper for prost-types to support serde annotations

## Local dev

Start plugin server
```
cargo run --bin helloworld
```

Send command from kong to run plugin
```
cargo run --bin kong_mock
```

debug macro
```
cargo expand --bin helloworld
```
 