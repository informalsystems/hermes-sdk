use cgp_core::prelude::*;

/// Allocate a TCP port that the full node process use for listening
#[async_trait]
pub trait CanReserveTcpPort: HasErrorType {
    async fn reserve_tcp_port(&self) -> Result<u16, Self::Error>;
}
