use kong_rust_pdk::{
    client::Client,
    ctx::Ctx,
    ip::Ip,
    nginx::Nginx,
    node::Node,
    pdk,
    router::Router,
    service::{request::ServiceRequest, response::ServiceResponse, Service},
};

use crate::{Log, Request, Response};

#[derive(Clone)]
pub struct Pdk {
    pub log: Log,
    pub request: Request,
    pub response: Response,
}

impl Pdk {
    pub(crate) fn new(request: Request) -> Self {
        Self {
            log: Log::default(),
            request,
            response: Response::new(),
        }
    }
}

impl pdk::Pdk for Pdk {
    fn client(&self) -> &dyn Client {
        todo!()
    }

    fn ctx(&self) -> &dyn Ctx {
        todo!()
    }

    fn log(&mut self) -> &mut dyn kong_rust_pdk::log::Log {
        &mut self.log
    }

    fn nginx(&self) -> &dyn Nginx {
        todo!()
    }

    fn request(&mut self) -> &mut dyn kong_rust_pdk::request::Request {
        &mut self.request
    }

    fn response(&mut self) -> &mut dyn kong_rust_pdk::response::Response {
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
