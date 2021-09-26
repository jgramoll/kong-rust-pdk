use std::io::Write;

use prost::{DecodeError, Message};

include!(concat!(env!("OUT_DIR"), "/kong_plugin_protocol.rs"));

// TODO should this happen on the transport layer
pub fn serialize_message<T: Message>(call: &T) -> Vec<u8> {
    let mut buf = Vec::new();
    let len = call.encoded_len();
    buf.reserve(len + 4);
    // TODO check error
    buf.write_all(&(len as u32).to_le_bytes()).unwrap();
    call.encode(&mut buf).unwrap();
    buf
}

pub fn deserialize_message<T: Message + Default>(buf: &[u8]) -> Result<T, DecodeError> {
    let (_len, bytes) = buf.split_at(4);
    T::decode(bytes)
}

#[cfg(test)]
mod tests {
    use super::{deserialize_message, serialize_message};

    use super::{rpc_call::Call, CmdStartInstance, RpcCall};

    use prost::Message;

    fn create_cmd_start_instance() -> RpcCall {
        let config = b"
            {
                \"message\": \"In a bottle\"
            }";

        RpcCall {
            sequence: 1,
            call: Some(Call::CmdStartInstance(CmdStartInstance {
                name: String::from("example-rust-plugin"),
                config: config.to_vec(),
            })),
        }
    }

    #[test]
    fn test_cmd() {
        let call_request = create_cmd_start_instance();
        let request_vector = serialize_message(&call_request);

        let request_deserialized_result = deserialize_message(&request_vector).unwrap();
        assert_eq!(call_request, request_deserialized_result);
    }

    #[test]
    fn test_cmd_2() {
        let call_request = create_cmd_start_instance();
        let request_vector = serialize_message(&call_request);

        assert_eq!(97, call_request.encoded_len());

        let (len, _bytes) = request_vector.split_at(4);
        assert_eq!(vec![97, 0, 0, 0], len);
    }
}
