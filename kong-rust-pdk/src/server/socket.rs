use std::io::Write;

use prost::Message;
use tokio::{io, net::UnixStream};

pub(crate) async fn send_to_stream<T: Message>(
    stream: &UnixStream,
    msg: &T,
) -> tokio::io::Result<()> {
    // send len + msg
    let mut buf = Vec::new();
    let len = msg.encoded_len();
    buf.reserve(len + 4);
    buf.write_all(&(len as u32).to_le_bytes()).unwrap();
    msg.encode(&mut buf).unwrap();

    loop {
        stream.writable().await?;

        match stream.try_write(&buf) {
            Ok(_n) => {
                break;
            }
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                continue;
            }
            Err(e) => {
                return Err(e);
            }
        }
    }
    Ok(())
}

pub(crate) async fn read_from_stream<T: Message + Default>(
    stream: &UnixStream,
) -> tokio::io::Result<T> {
    let mut buf = vec![0; 1024];

    loop {
        stream.readable().await?;

        match stream.try_read(&mut buf) {
            Ok(n) => {
                if n > 0 {
                    buf.truncate(n);
                    break;
                }
            }
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                continue;
            }
            Err(e) => {
                return Err(e);
            }
        }
    }

    // read len + msg
    let (_len, bytes) = buf.split_at(4);
    let ret = T::decode(bytes)?;
    Ok(ret)
}
