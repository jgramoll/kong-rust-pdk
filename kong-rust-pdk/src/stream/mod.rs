use std::sync::Arc;

use tokio::net::UnixStream;

pub(crate) mod read;
pub(crate) mod write;

#[derive(Clone, Debug)]
pub(crate) struct Stream(pub(crate) Arc<UnixStream>);

impl Stream {
    pub fn new(stream: UnixStream) -> Self {
        Self(Arc::new(stream))
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;

    pub(crate) fn new_stream() -> std::io::Result<(Stream, Stream)> {
        let t = UnixStream::pair();
        t.map(|streams| (Stream::new(streams.0), Stream::new(streams.1)))
    }
}
