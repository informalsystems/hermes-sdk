use cgp::prelude::*;

#[cgp_component {
  name: TcpPortReserverComponent,
  provider: TcpPortReserver,
  context: Runtime,
}]
/// Allocate a TCP port that the full node process use for listening
#[async_trait]
pub trait CanReserveTcpPort: Async + HasErrorType {
    async fn reserve_tcp_port(&self) -> Result<u16, Self::Error>;
}
