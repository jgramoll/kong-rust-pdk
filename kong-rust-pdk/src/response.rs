use std::{collections::BTreeMap, io, iter::Map};

use async_trait::async_trait;

use crate::{stream::Stream, Error};

#[async_trait]
pub trait Response: Send + Sync {
    async fn get_status(&self) -> io::Result<usize>;
    async fn get_header(&self, name: &str) -> Result<String, Error>;
    async fn get_headers(&self, max_headers: usize) -> Result<Map<String, Vec<String>>, Error>;
    async fn get_source(&self) -> Result<String, Error>;
    async fn set_status(&mut self, status: usize) -> Result<(), Error>;
    async fn set_header(&mut self, key: &str, value: &str) -> Result<(), Error>;
    async fn add_header(&self, key: String, value: String) -> Result<(), Error>;
    async fn clear_header(&self, key: String) -> Result<(), Error>;
    async fn set_headers(&self, headers: Map<String, Vec<String>>) -> Result<(), Error>;
    async fn exit(&self, status: usize, body: String, headers: ()) -> io::Result<()>;
    async fn exit_status(&mut self, status: usize) -> io::Result<()>;
}

#[derive(Debug, Clone)]
pub struct PbServerResponse {
    stream: Stream,
}

impl PbServerResponse {
    pub(crate) fn new(stream: Stream) -> Self {
        Self { stream }
    }
}

// impl PbServerResponse {
//     pub async fn get_header(&self, name: String) -> Result<String, Error> {
//         println!("Getting header {}", name);
//         // TODO error type
//         // TODO type
//         self.stream
//             .write_method_with_args("kong.response.get_header", &pb::String { v: name })
//             .await
//             .unwrap();
//         Ok(String::new())
//     }

//     //map[string][]string
//     pub async fn get_headers(&self, name: String) -> Result<String, Error> {
//         println!("Getting header {}", name);
//         // TODO error type
//         // TODO type
//         self.stream
//             .write_method_with_args("kong.response.get_headers", &pb::String { v: name })
//             .await
//             .unwrap();
//         Ok(String::new())
//     }
// }

#[async_trait]
impl Response for PbServerResponse {
    async fn get_status(&self) -> io::Result<usize> {
        let status = self.stream.write_method("kong.response.get_status").await?;
        Ok(status)
    }

    async fn get_header(&self, _name: &str) -> Result<String, Error> {
        todo!()
    }

    async fn get_headers(&self, _max_headers: usize) -> Result<Map<String, Vec<String>>, Error> {
        todo!()
    }

    async fn get_source(&self) -> Result<String, Error> {
        todo!()
    }

    async fn set_status(&mut self, _status: usize) -> Result<(), Error> {
        todo!()
    }
    async fn set_header(&mut self, _name: &str, _valuee: &str) -> Result<(), Error> {
        // TODO error type
        // TODO type
        // self.stream
        //     .write_method_with_args("kong.response.set_header", &pb::String { v: name })
        //     .await
        //     .unwrap();
        todo!()
        // Ok(())
    }

    async fn add_header(&self, _key: String, _value: String) -> Result<(), Error> {
        todo!()
    }

    async fn clear_header(&self, _key: String) -> Result<(), Error> {
        todo!()
    }

    async fn set_headers(&self, _headers: Map<String, Vec<String>>) -> Result<(), Error> {
        todo!()
    }

    //map[string][]string
    async fn exit(&self, status: usize, body: String, _headers: ()) -> io::Result<()> {
        // TODO error type
        // TODO type

        self.stream
            .write_method_with_args(
                "kong.response.exit",
                &pb::ExitArgs {
                    status: status as i32,
                    body,
                    headers: Some(prost_types::Struct {
                        fields: BTreeMap::new(),
                        //asd
                    }),
                },
            )
            .await?;

        Ok(())
    }

    async fn exit_status(&mut self, _status: usize) -> io::Result<()> {
        todo!()
    }
}
