use std::future::Future;

use kong_rust_pdk::{server, Plugin};
use tokio::runtime::Runtime;

pub(crate) fn server_start<T, F>(
    config: T,
    version: &'static str,
    priority: usize,
    future: F,
) -> Result<(), Box<dyn std::error::Error>>
where
    T: 'static + Plugin,
    F: Future + Send + 'static,
    F::Output: Send + 'static,
{
    let rt = Runtime::new().unwrap();
    rt.block_on(async move {
        tokio::spawn(async move {
            server::start::<T>(version, priority).await.unwrap();
        });

        tokio::spawn(future).await?;

        Ok(())
    })
}
