use strum::{EnumString, IntoStaticStr};

use crate::async_trait;
use crate::{stream::Stream, Result};

pub struct AuthenticatedCredential;
pub struct Consumer;

#[async_trait]
pub trait Client {
    // kong.client().get_ip() returns the remote address of the client making the request.
    // This will always return the address of the client directly connecting to Kong.
    // That is, in cases when a load balancer is in front of Kong, this function will
    // return the load balancer’s address, and not that of the downstream client.
    async fn get_ip(&self) -> Result<String>;

    // kong.clien().get_forwarded_ip() returns the remote address of the client making the request.
    // Unlike kong.client.get_ip, this function will consider forwarded addresses in cases
    // when a load balancer is in front of Kong. Whether this function returns a forwarded
    // address or not depends on several Kong configuration parameters:
    //
    //   - trusted_ips
    //   - real_ip_header
    //   - real_ip_recursive
    async fn get_forwarded_ip(&self) -> Result<String>;

    // kong.client().get_port() returns the remote port of the client making the request.
    // This will always return the port of the client directly connecting to Kong.
    // That is, in cases when a load balancer is in front of Kong, this function
    // will return load balancer’s port, and not that of the downstream client.
    async fn get_port(&self) -> Result<i32>;

    // kong.client().get_forwarded_port() returns the remote port of the client making the request.
    // Unlike kong.client.get_port, this function will consider forwarded ports in cases
    // when a load balancer is in front of Kong. Whether this function returns a forwarded
    // port or not depends on several Kong configuration parameters:
    //
    //   - trusted_ips
    //   - real_ip_header
    //   - real_ip_recursive
    async fn get_forwarded_port(&self) -> Result<i32>;

    async fn get_credential(&self) -> Result<AuthenticatedCredential>;
    async fn load_consumer(&self, consumer_id: String, by_username: bool) -> Result<Consumer>;
    async fn get_consumer(&self) -> Result<Consumer>;
    async fn authenticate(
        &self,
        consumer: Consumer,
        credential: AuthenticatedCredential,
    ) -> Result<()>;
    async fn get_protocol(&self, allow_terminated: bool) -> Result<String>;
}

#[derive(Debug, PartialEq, IntoStaticStr, EnumString)]
pub(crate) enum Methods {
    #[strum(serialize = "kong.client.get_ip")]
    GetIp,
    #[strum(serialize = "kong.client.get_forwarded_ip")]
    GetForwardedIp,
    #[strum(serialize = "kong.client.get_port")]
    GetPort,
    #[strum(serialize = "kong.client.get_forwarded_port")]
    GetForwardedPort,
    #[strum(serialize = "kong.client.get_credential")]
    GetCredential,
    #[strum(serialize = "kong.client.load_consumer")]
    LoadConsumer,
    #[strum(serialize = "kong.client.get_consumer")]
    GetConsumer,
    #[strum(serialize = "kong.client.authenticate")]
    Authenticate,
    #[strum(serialize = "kong.client.get_protocol")]
    GetProtocol,
}

struct PbClient {
    stream: Stream,
}

#[async_trait]
impl Client for PbClient {
    async fn get_ip(&self) -> Result<String> {
        self.stream.ask_string(Methods::GetIp.into()).await
    }

    async fn get_forwarded_ip(&self) -> Result<String> {
        self.stream.ask_string(Methods::GetForwardedIp.into()).await
    }

    async fn get_port(&self) -> Result<i32> {
        self.stream.ask_int(Methods::GetPort.into()).await
    }

    async fn get_forwarded_port(&self) -> Result<i32> {
        self.stream.ask_int(Methods::GetForwardedPort.into()).await
    }

    async fn get_credential(&self) -> Result<AuthenticatedCredential> {
        todo!()
    }

    async fn load_consumer(&self, _consumer_id: String, _by_username: bool) -> Result<Consumer> {
        todo!()
    }

    async fn get_consumer(&self) -> Result<Consumer> {
        todo!()
    }

    async fn authenticate(
        &self,
        _consumer: Consumer,
        _credential: AuthenticatedCredential,
    ) -> Result<()> {
        todo!()
    }

    async fn get_protocol(&self, _allow_terminated: bool) -> Result<String> {
        todo!()
    }
}
