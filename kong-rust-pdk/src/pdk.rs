use crate::{
    request::{PbServerRequest, Request},
    response::{PbServerResponse, Response},
    stream::Stream,
};

pub trait Pdk: Clone + Send + Sync {
    fn request(&mut self) -> &mut dyn Request;
    fn response(&mut self) -> &mut dyn Response;
}

#[derive(Debug, Clone)]
pub(crate) struct StreamPdk {
    stream: Stream,
    request: PbServerRequest,
    response: PbServerResponse,
}

impl StreamPdk {
    pub(crate) fn new(stream: Stream) -> Self {
        Self {
            stream: stream.clone(),
            request: PbServerRequest::new(stream.clone()),
            response: PbServerResponse::new(stream),
        }
    }
}

impl Pdk for StreamPdk {
    fn request(&mut self) -> &mut dyn Request {
        &mut self.request
    }
    fn response(&mut self) -> &mut dyn Response {
        &mut self.response
    }
}
