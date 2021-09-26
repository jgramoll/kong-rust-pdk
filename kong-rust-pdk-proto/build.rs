use std::io::Result;

fn main() -> Result<()> {
    // let build_client = cfg!(feature = "build_client");
    // let build_server = cfg!(feature = "build_server");

    // .build_client(build_client)
    // .build_server(build_server)

    prost_build::Config::new()
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
        .extern_path(".google.protobuf.Value", "::serde_prost_types::Value")
        .compile_protos(&["proto/pluginserver.proto"], &["proto"])?;
    Ok(())
}
