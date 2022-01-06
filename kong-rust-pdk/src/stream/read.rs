use prost::Message;
use tokio::io;

use super::Stream;

impl Stream {
    // read bytes from stream to given array
    pub(crate) async fn read(&self, mut out: &mut [u8]) -> tokio::io::Result<usize> {
        loop {
            self.0.readable().await?;
            match self.0.try_read(&mut out) {
                Ok(0) => return Err(std::io::Error::from(std::io::ErrorKind::ConnectionAborted)),
                Ok(n) => {
                    log::trace!("Read {} bytes", n);
                    if n > 0 {
                        break Ok(n);
                    }
                }
                Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                    log::trace!("read would block");
                    continue;
                }
                Err(e) => {
                    break Err(e);
                }
            }
        }
    }

    async fn read_i32(&self) -> tokio::io::Result<i32> {
        let mut bytes = [0; 4];
        let len = self.read(&mut bytes).await?;
        debug_assert!(len == 4);
        Ok(i32::from_le_bytes(bytes))
    }

    pub(crate) async fn read_frame(&self) -> tokio::io::Result<Vec<u8>> {
        // read len + msg
        let len = self.read_i32().await? as usize;
        if len == 0 {
            return Ok(vec![]);
        }
        log::trace!("Read frame size: {}", len);

        let mut buf = vec![0; len];
        let read_len = self.read(&mut buf).await?;
        debug_assert_eq!(read_len, len);

        let (bytes, _) = buf.split_at(read_len);
        Ok(bytes.to_vec())
    }

    #[cfg(any(feature = "test_client", test))]
    pub(crate) async fn read_method(&self) -> Result<String, Box<dyn std::error::Error>> {
        let bytes = self.read_frame().await?;
        let s = std::str::from_utf8(&bytes).map(String::from)?;
        Ok(s)
    }

    pub(crate) async fn read_message<T: Message + Default>(&self) -> tokio::io::Result<T> {
        let bytes = self.read_frame().await?;
        let t = T::decode(&*bytes)?;
        Ok(t)
    }
}

#[cfg(test)]
mod tests {
    use prost::Message;

    use crate::stream::{tests::new_stream, write::tests::write_to_stream};

    #[tokio::test]
    async fn test_read_number() -> Result<(), Box<dyn std::error::Error>> {
        let (left, right) = new_stream()?;
        write_to_stream(&left, &7_i32.to_le_bytes()).await?;

        let res = right.read_i32().await?;
        assert_eq!(7, res);

        Ok(())
    }

    #[tokio::test]
    async fn test_read_method() -> Result<(), Box<dyn std::error::Error>> {
        let str = "my.special.method";
        let bytes = str.as_bytes();

        let (left, right) = new_stream()?;
        write_to_stream(&left, &(bytes.len() as i32).to_le_bytes()).await?;
        write_to_stream(&left, bytes).await?;

        let res = right.read_method().await?;
        assert_eq!(str, res);

        Ok(())
    }

    #[tokio::test]
    async fn test_read_frame() -> Result<(), Box<dyn std::error::Error>> {
        let str = "thisisacode";
        let bytes = str.as_bytes();

        let (left, right) = new_stream()?;
        write_to_stream(&left, &(bytes.len() as i32).to_le_bytes()).await?;
        write_to_stream(&left, bytes).await?;

        let res = right.read_frame().await?;
        assert_eq!(bytes, res);

        Ok(())
    }

    #[tokio::test]
    async fn test_read_message() -> Result<(), Box<dyn std::error::Error>> {
        let v = String::from("my message");
        let msg = pb::String { v };
        let bytes = msg.encode_to_vec();

        let (left, right) = new_stream()?;
        write_to_stream(&left, &(bytes.len() as i32).to_le_bytes()).await?;
        write_to_stream(&left, &bytes).await?;

        let res = right.read_message().await?;
        assert_eq!(msg, res);

        Ok(())
    }
}
