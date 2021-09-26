include!(concat!(env!("OUT_DIR"), "/kong_plugin_protocol.rs"));

#[cfg(test)]
mod tests {
    use super::{rpc_call::Call, CmdStartInstance, RpcCall};

    use prost::Message;

    fn create_cmd_start_instance(plugin_name: std::string::String, config: Vec<u8>) -> RpcCall {
        RpcCall {
            sequence: 1,
            call: Some(Call::CmdStartInstance(CmdStartInstance {
                name: plugin_name,
                config,
            })),
        }
    }

    fn serialize_cmd_start_instance(call: &RpcCall) -> Vec<u8> {
        let mut buf = Vec::new();
        buf.reserve(call.encoded_len());
        call.encode_length_delimited(&mut buf).unwrap();
        buf
    }

    fn deserialize_cmd_start_instance(buf: &[u8]) -> Result<RpcCall, prost::DecodeError> {
        let call = RpcCall::decode(buf)?;
        Ok(call)
    }

    #[test]
    fn test_cmd() {
        let plugin_name = String::from("example-rust-plugin");
        let config = String::from(
            r#"
            {
                "message": "In a bottle"
            }"#,
        );

        let call_request = create_cmd_start_instance(plugin_name, config.clone().into_bytes());
        let request_vector = serialize_cmd_start_instance(&call_request);

        let request_deserialized_result = deserialize_cmd_start_instance(&request_vector).unwrap();
        match request_deserialized_result.call.unwrap() {
            Call::CmdStartInstance(cmd) => {
                assert_eq!(config, std::str::from_utf8(&cmd.config).unwrap());
            }
            _ => panic!("Invalid deserialize type"),
        };
    }
}
