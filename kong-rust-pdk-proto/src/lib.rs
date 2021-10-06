include!(concat!(env!("OUT_DIR"), "/kong_plugin_protocol.rs"));

impl Kv {
    pub fn new(k: std::string::String, v: Option<std::string::String>) -> Self {
        Self {
            k,
            v: v.map(|v| serde_prost_types::Value {
                kind: Some(serde_prost_types::value::Kind::StringValue(v)),
            }),
        }
    }
}
