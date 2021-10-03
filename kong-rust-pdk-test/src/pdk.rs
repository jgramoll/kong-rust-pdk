use kong_rust_pdk::pdk;

use crate::{Request, Response};

#[derive(Clone)]
pub(crate) struct Pdk {
    pub(crate) request: Request,
    pub(crate) response: Response,
}

impl Pdk {
    pub(crate) fn new(request: Request) -> Self {
        Self {
            request,
            response: Response::new(),
        }
    }
}

impl pdk::Pdk for Pdk {
    fn response(&mut self) -> &mut (dyn kong_rust_pdk::response::Response) {
        &mut self.response
    }

    fn request(&mut self) -> &mut (dyn kong_rust_pdk::request::Request) {
        &mut self.request
    }
}
