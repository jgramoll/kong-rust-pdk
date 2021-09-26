use crate::Error;

#[derive(Debug, Clone)]
pub struct Response {}

impl Response {
    pub fn set_header(&self, _name: &str, _value: &str) -> Result<(), Error> {
        println!("Setting header...");
        Ok(())
    }
}
