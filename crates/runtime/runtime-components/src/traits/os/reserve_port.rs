use cgp::prelude::*;

#[cgp_component {
  provider: TcpPortReserver,
  context: Runtime,
}]
/// Allocate a TCP port that the full node process use for listening
#[async_trait]
pub trait CanReserveTcpPort: Async + HasAsyncErrorType {
    async fn reserve_tcp_port(&self) -> Result<u16, Self::Error>;
}
