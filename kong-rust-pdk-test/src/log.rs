use kong_rust_pdk::{async_trait, Result};

#[derive(Default, Clone)]
pub struct Log {
    pub alerts: Vec<String>,
}

#[async_trait]
impl kong_rust_pdk::log::Log for Log {
    async fn alert(&mut self, args: String) -> Result<()> {
        // TODO better mock without mut?
        self.alerts.push(args);
        Ok(())
    }
    async fn crit(&self, _args: String) -> Result<()> {
        todo!()
    }
    async fn err(&self, _args: String) -> Result<()> {
        todo!()
    }
    async fn warn(&self, _args: String) -> Result<()> {
        todo!()
    }
    async fn notice(&self, _args: String) -> Result<()> {
        todo!()
    }
    async fn info(&self, _args: String) -> Result<()> {
        todo!()
    }
    async fn debug(&self, _args: String) -> Result<()> {
        todo!()
    }
    async fn serialize(&self) -> Result<String> {
        todo!()
    }
}
