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
use kong_rust_pdk::{macros::*, pdk::Pdk, server, Error, Plugin};

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

impl Default for Config {
    fn default() -> Self {
        Self {
            message: String::from("default message"),
        }
    }
}

#[plugin_impl]
impl Plugin for Config {
    async fn access<T: Pdk>(&self, kong: &mut T) -> Result<(), Error> {
        let method = kong.request().get_method().await?;

        kong.response().set_status(204).await?;

        kong.response()
            .set_header("x-hello-from-rust", &method)
            .await?;

        Ok(())
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
 