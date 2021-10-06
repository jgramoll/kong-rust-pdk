use prost::Message;
use tokio::io;

use super::Stream;

impl Stream {
    async fn write(&self, buf: &[u8]) -> tokio::io::Result<usize> {
        loop {
            self.0.writable().await?;

            match self.0.try_write(buf) {
                Ok(n) => {
                    log::trace!("wrote {} bytes", n);
                    break Ok(n);
                }
                Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                    log::trace!("write would block");
                    continue;
                }
                Err(e) => {
                    break Err(e);
                }
            }
        }
    }

    pub(crate) async fn write_frame(&self, buf: &[u8]) -> tokio::io::Result<usize> {
        // send len + msg
        let len = buf.len();
        let res1 = self.write(&(len as u32).to_le_bytes()).await?;
        let res2 = self.write(buf).await?;

        Ok(res1 + res2)
    }

    pub(crate) async fn write_message<T: Message>(&self, msg: &T) -> tokio::io::Result<usize> {
        self.write_frame(&msg.encode_to_vec()).await
    }
}

#[cfg(test)]
mod tests {
    use std::io::Write;

    use prost::Message;

    use crate::stream::tests::new_stream;

    #[tokio::test]
    async fn test_write() -> Result<(), Box<dyn std::error::Error>> {
        let (left, right) = new_stream()?;
        let len = left.write(&7_i32.to_le_bytes()).await?;
        assert_eq!(4, len);

        let mut buf = [0_u8; 4];
        let len = right.read(&mut buf).await?;
        assert_eq!([7, 0, 0, 0], buf);
        assert_eq!(4, len);

        Ok(())
    }

    #[tokio::test]
    async fn test_write_frame() -> Result<(), Box<dyn std::error::Error>> {
        let str: &str = "thisisacode";
        let bytes: &[u8] = str.as_bytes();
        let expected_len: usize = bytes.len() + 4;

        let (left, right) = new_stream()?;
        let len = left.write_frame(bytes).await?;
        assert_eq!(expected_len, len);

        let mut buf = vec![0; expected_len];
        right.read(&mut buf).await?;

        let mut expected: Vec<u8> = Vec::new();
        expected.write_all(&11_i32.to_le_bytes())?;
        expected.write_all(bytes)?;
        assert_eq!(expected, buf);

        Ok(())
    }

    #[tokio::test]
    async fn test_write_message() -> Result<(), Box<dyn std::error::Error>> {
        let msg = pb::String {
            v: String::from("foo"),
        };
        let expected_len = msg.encoded_len() + 4;

        let (left, right) = new_stream()?;
        let len = left.write_message(&msg).await?;
        assert_eq!(expected_len, len);

        let mut bytes = vec![0; expected_len];
        right.read(&mut bytes).await?;

        let mut expected: Vec<u8> = Vec::new();
        expected.reserve(expected_len);
        expected.write_all(&(msg.encoded_len() as i32).to_le_bytes())?;
        expected.write_all(&msg.encode_to_vec())?;
        assert_eq!(expected, bytes);

        Ok(())
    }

    #[tokio::test]
    async fn test_write_method() -> Result<(), Box<dyn std::error::Error>> {
        let method = "foo";
        let method_bytes = method.as_bytes();
        let expected_len = 4 + method_bytes.len() + 4;

        let (left, right) = new_stream()?;
        let len = left.write_method(method).await?;
        assert_eq!(expected_len, len);

        let mut bytes = vec![0; expected_len];
        right.read(&mut bytes).await?;

        let mut expected: Vec<u8> = Vec::new();
        expected.reserve(expected_len);
        expected.write_all(&(method_bytes.len() as i32).to_le_bytes())?;
        expected.write_all(method_bytes)?;
        expected.write_all(&0_i32.to_le_bytes())?;
        assert_eq!(expected, bytes);

        Ok(())
    }

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
