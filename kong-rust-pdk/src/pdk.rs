use crate::{
    client::Client,
    ctx::Ctx,
    ip::Ip,
    log::{Log, PbLog},
    nginx::Nginx,
    node::Node,
    request::{PbServerRequest, Request},
    response::{PbServerResponse, Response},
    router::Router,
    service::{request::ServiceRequest, response::ServiceResponse, Service},
    stream::Stream,
};

pub trait Pdk: Clone + Send + Sync {
    fn client(&self) -> &dyn Client;
    fn ctx(&self) -> &dyn Ctx;
    fn log(&mut self) -> &mut dyn Log;
    fn nginx(&self) -> &dyn Nginx;
    fn request(&mut self) -> &mut dyn Request;
    fn response(&mut self) -> &mut dyn Response;
    fn router(&self) -> &dyn Router;
    fn ip(&self) -> &dyn Ip;
    fn node(&self) -> &dyn Node;
    fn service(&self) -> &dyn Service;
    fn service_request(&self) -> &dyn ServiceRequest;
    fn service_response(&self) -> &dyn ServiceResponse;
}

#[derive(Clone)]
pub(crate) struct StreamPdk {
    stream: Stream,
    log: PbLog,
    request: PbServerRequest,
    response: PbServerResponse,
}

impl StreamPdk {
    pub(crate) fn new(stream: Stream) -> Self {
        Self {
            stream: stream.clone(),
            log: PbLog::new(stream.clone()),
            request: PbServerRequest::new(stream.clone()),
            response: PbServerResponse::new(stream),
        }
    }
}

impl Pdk for StreamPdk {
    fn client(&self) -> &dyn Client {
        todo!()
    }
    fn ctx(&self) -> &dyn Ctx {
        todo!()
    }

    fn log(&mut self) -> &mut dyn Log {
        &mut self.log
    }

    fn nginx(&self) -> &dyn Nginx {
        todo!()
    }

    fn request(&mut self) -> &mut dyn Request {
        &mut self.request
    }

    fn response(&mut self) -> &mut dyn Response {
        &mut self.response
    }

    fn router(&self) -> &dyn Router {
        todo!()
    }

    fn ip(&self) -> &dyn Ip {
        todo!()
    }

    fn node(&self) -> &dyn Node {
        todo!()
    }

    fn service(&self) -> &dyn Service {
        todo!()
    }

    fn service_request(&self) -> &dyn ServiceRequest {
        todo!()
    }

    fn service_response(&self) -> &dyn ServiceResponse {
        todo!()
    }
}
