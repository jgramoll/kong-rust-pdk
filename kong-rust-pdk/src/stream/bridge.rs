use prost::Message;

use crate::Result;

use super::Stream;

impl Stream {
    pub(crate) async fn write_method(&self, method: &str) -> tokio::io::Result<usize> {
        let res1 = self.write_frame(method.as_bytes()).await?;
        // empty frame for 0 args
        let res2 = self.write_frame(&[]).await?;

        Ok(res1 + res2)
    }

    async fn write_method_with_args<T: Message>(
        &self,
        method: &str,
        args: &T,
    ) -> tokio::io::Result<usize> {
        let res1 = self.write_frame(method.as_bytes()).await?;
        let res2 = self.write_frame(&args.encode_to_vec()).await?;

        Ok(res1 + res2)
    }

    pub(crate) async fn ask<T: prost::Message>(&self, method: &str, args: &T) -> Result<()> {
        self.write_method_with_args(method, args).await?;
        self.read_frame().await?;
        Ok(())
    }

    pub(crate) async fn send_string(&self, method: &str, v: String) -> Result<()> {
        self.ask(method, &pb::String { v }).await
    }

    pub(crate) async fn send_int(&self, method: &str, v: i32) -> Result<()> {
        self.ask(method, &pb::Int { v }).await
    }

    pub(crate) async fn ask_string(&self, method: &str) -> Result<String> {
        self.write_method(method).await?;
        let s = self.read_message::<pb::String>().await?;
        Ok(s.v)
    }

    pub(crate) async fn ask_int(&self, method: &str) -> Result<i32> {
        self.write_method(method).await?;
        let s = self.read_message::<pb::Int>().await?;
        Ok(s.v)
    }
}
