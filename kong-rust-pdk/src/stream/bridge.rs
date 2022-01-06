use std::str::FromStr;

use http::{header::HeaderName, HeaderMap, HeaderValue};
use prost::Message;

use crate::Result;

use super::Stream;

impl Stream {
    pub(crate) async fn write_method(&self, method: &str) -> tokio::io::Result<usize> {
        let res1 = self.write_frame(method.as_bytes()).await?;
        // empty frame for 0 args
        let res2 = self.write_frame(&[]).await?;

        Ok(res1 + res2)
    }

    async fn write_method_with_args<T: Message>(
        &self,
        method: &str,
        args: &T,
    ) -> tokio::io::Result<usize> {
        let res1 = self.write_frame(method.as_bytes()).await?;
        let res2 = self.write_frame(&args.encode_to_vec()).await?;

        Ok(res1 + res2)
    }

    pub(crate) async fn ask<T: prost::Message>(&self, method: &str, args: &T) -> Result<()> {
        self.write_method_with_args(method, args).await?;
        self.read_frame().await?;
        Ok(())
    }

    pub(crate) async fn ask_message<T: prost::Message, R: prost::Message + Default>(
        &self,
        method: &str,
        args: &T,
    ) -> Result<R> {
        self.write_method_with_args(method, args).await?;
        let out = self.read_message::<R>().await?;
        Ok(out)
    }

    #[allow(dead_code)]
    pub(crate) async fn send_string(&self, method: &str, v: String) -> Result<()> {
        self.ask(method, &pb::String { v }).await
    }

    pub(crate) async fn send_int(&self, method: &str, v: i32) -> Result<()> {
        self.ask(method, &pb::Int { v }).await
    }

    pub(crate) async fn ask_string(&self, method: &str) -> Result<String> {
        self.write_method(method).await?;
        let s = self.read_message::<pb::String>().await?;
        Ok(s.v)
    }

    pub(crate) async fn ask_string_with_args<T: prost::Message>(
        &self,
        method: &str,
        args: &T,
    ) -> Result<String> {
        self.write_method_with_args(method, args).await?;
        let s = self.read_message::<pb::String>().await?;
        Ok(s.v)
    }

    pub(crate) async fn ask_int(&self, method: &str) -> Result<i32> {
        self.write_method(method).await?;
        let s = self.read_message::<pb::Int>().await?;
        Ok(s.v)
    }

    #[allow(dead_code)]
    pub(crate) async fn ask_int_with_args<T: prost::Message>(
        &self,
        method: &str,
        args: &T,
    ) -> Result<i32> {
        self.write_method_with_args(method, args).await?;
        let s = self.read_message::<pb::Int>().await?;
        Ok(s.v)
    }

    pub(crate) async fn ask_number(&self, method: &str) -> Result<f64> {
        self.write_method(method).await?;
        let s = self.read_message::<pb::Number>().await?;
        Ok(s.v)
    }

    pub fn unwrap_headers(st: serde_prost_types::Struct) -> Result<HeaderMap> {
        let mut ret = HeaderMap::default();

        for (name, v) in st.fields {
            if let Some(kind) = v.kind {
                let name = HeaderName::from_str(&name).unwrap();
                let value = match kind {
                    serde_prost_types::value::Kind::NullValue(_) => todo!(),
                    serde_prost_types::value::Kind::NumberValue(n) => {
                        HeaderValue::from_str(&n.to_string())
                    }
                    serde_prost_types::value::Kind::StringValue(str) => HeaderValue::from_str(&str),
                    serde_prost_types::value::Kind::BoolValue(b) => {
                        HeaderValue::from_str(&b.to_string())
                    }
                    serde_prost_types::value::Kind::StructValue(_) => todo!(),
                    serde_prost_types::value::Kind::ListValue(l) => {
                        for _v in l.values {
                            // TODO how to get HeaderValue
                        }
                        todo!()
                    }
                }
                .unwrap();

                ret.append(name, value);
            }
        }

        Ok(ret)
    }
}

#[cfg(test)]
mod tests {
    use std::io::Write;

    use prost::Message;

    use crate::stream::tests::new_stream;

    #[tokio::test]
    async fn test_write_method_with_args() -> Result<(), Box<dyn std::error::Error>> {
        let method = "foo";
        let method_bytes = method.as_bytes();
        let args = pb::String {
            v: String::from("bar"),
        };
        let expected_len = 4 + method_bytes.len() + 4 + args.encoded_len();

        let (left, right) = new_stream()?;
        let len = left.write_method_with_args(method, &args).await?;
        assert_eq!(expected_len, len);

        let mut bytes = vec![0; expected_len];
        right.read(&mut bytes).await?;

        let mut expected: Vec<u8> = Vec::new();
        expected.reserve(expected_len);
        expected.write_all(&(method_bytes.len() as i32).to_le_bytes())?;
        expected.write_all(method_bytes)?;
        expected.write_all(&(args.encoded_len() as i32).to_le_bytes())?;
        expected.write_all(&args.encode_to_vec())?;
        assert_eq!(expected, bytes);

        Ok(())
    }
}
