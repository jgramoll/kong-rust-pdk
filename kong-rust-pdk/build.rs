fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        // .build_server(false)
        .type_attribute("RpcCall", "#[derive(serde::Serialize, serde::Deserialize)]")
        .type_attribute(
            "RpcCall.call",
            "#[derive(serde::Serialize, serde::Deserialize)]",
        )
        .type_attribute(
            "CmdGetPluginNames",
            "#[derive(serde::Serialize, serde::Deserialize)]",
        )
        .type_attribute(
            "CmdGetPluginInfo",
            "#[derive(serde::Serialize, serde::Deserialize)]",
        )
        .type_attribute(
            "CmdStartInstance",
            "#[derive(serde::Serialize, serde::Deserialize)]",
        )
        .type_attribute(
            "CmdGetInstanceStatus",
            "#[derive(serde::Serialize, serde::Deserialize)]",
        )
        .type_attribute(
            "CmdCloseInstance",
            "#[derive(serde::Serialize, serde::Deserialize)]",
        )
        .type_attribute(
            "CmdHandleEvent",
            "#[derive(serde::Serialize, serde::Deserialize)]",
        )
        .type_attribute(
            "RpcReturn",
            "#[derive(serde::Serialize, serde::Deserialize)]",
        )
        .type_attribute(
            "RpcReturn.return",
            "#[derive(serde::Serialize, serde::Deserialize)]",
        )
        .type_attribute(
            "InstanceStatus",
            "#[derive(serde::Serialize, serde::Deserialize)]",
        )
        .type_attribute(
            "PluginInfo",
            "#[derive(serde::Serialize, serde::Deserialize)]",
        )
        .type_attribute(
            "PluginNames",
            "#[derive(serde::Serialize, serde::Deserialize)]",
        )
        .type_attribute(
            ".google.protobuf.Value",
            "#[derive(serde::Serialize, serde::Deserialize)]",
        )
        // .extern_path(".google.protobuf.Value", "::serde_json::Value")
        .extern_path(".google.protobuf.Value", "::serde_prost_types::Value")
        // .type_attribute(
        //     "google.protobuf.Value",
        //     "#[derive(serde::Serialize, serde::Deserialize)]",
        // )
        .compile(&["proto/pluginsocket.proto"], &["proto"])?;
    Ok(())
}
