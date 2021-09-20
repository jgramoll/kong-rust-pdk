use crate::Error;

#[derive(Debug, Clone)]
pub struct Response {}

impl Response {
    #[allow(clippy::clippy::unnecessary_wraps)]
    pub fn set_header(&self, name: &str, value: &str) -> Result<(), Error> {
        println!("Setting header...");
        Ok(())
    }
}
