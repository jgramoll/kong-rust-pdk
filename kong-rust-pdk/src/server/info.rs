use std::collections::HashMap;

use serde::Serialize;

use crate::{server::get_socket_path, PluginConfig, PluginSchema};

use super::get_name;

const PLUGIN_PROTOCOL: &str = "ProtoBuf:1";

// Types for serializing the dump response
#[derive(Serialize)]
#[serde(rename_all = "PascalCase")]
pub(crate) struct ServerInfo {
    protocol: String,
    socket_path: String,
    plugins: Vec<PluginInfo>,
}

// TODO can we use pb::PluginInfo
// TODO can we use google.protobuf.Value
#[derive(Serialize)]
#[serde(rename_all = "PascalCase")]
struct PluginInfo {
    name: String,
    phases: Vec<String>,
    version: String,
    priority: usize,
    schema: PluginInfoPlugins,
}

// TODO can we clean up types
#[derive(Serialize)]
struct PluginInfoPlugins {
    name: String,
    fields: Vec<PluginInfoSchema>,
}

#[derive(Serialize)]
struct PluginInfoSchema {
    config: PluginInfoRecord,
}

#[derive(Serialize)]
struct PluginInfoRecord {
    r#type: String,
    fields: Vec<PluginInfoType>,
}

#[derive(Serialize)]
#[serde(untagged)]
enum PluginInfoType {
    String(String),
    HashMap(HashMap<String, PluginInfoType>),
    // Vec(Vec<PluginInfoType>),
}

pub(crate) struct ServerInfoBuilder {
    version: String,
    priority: usize,
    #[allow(dead_code)]
    schema: String,
    phases: Vec<String>,
}

impl ServerInfoBuilder {
    pub(crate) fn new<T: PluginConfig + PluginSchema>(
        version: String,
        priority: usize,
    ) -> ServerInfoBuilder {
        ServerInfoBuilder {
            version,
            priority,
            schema: T::get_schema(),
            phases: T::get_phases(),
        }
    }

    pub(crate) fn build(self) -> ServerInfo {
        let plugin = PluginInfo {
            name: get_name(),
            phases: self.phases.clone(),
            version: self.version.clone(),
            priority: self.priority,
            schema: Self::get_schema(),
        };
        ServerInfo {
            protocol: String::from(PLUGIN_PROTOCOL),
            socket_path: get_socket_path(),
            plugins: vec![plugin],
        }
    }

    // TODO clean this up plz
    fn get_schema() -> PluginInfoPlugins {
        let mut message_type = HashMap::new();
        message_type.insert(
            String::from("type"),
            PluginInfoType::String(String::from("string")),
        );

        let mut message_field = HashMap::new();
        message_field.insert(
            String::from("message"),
            PluginInfoType::HashMap(message_type),
        );

        // TODO fields from frd schema
        let config = PluginInfoRecord {
            r#type: String::from("record"),
            fields: vec![PluginInfoType::HashMap(message_field)],
        };

        PluginInfoPlugins {
            name: get_name(),
            fields: vec![PluginInfoSchema { config }],
        }
    }
}
