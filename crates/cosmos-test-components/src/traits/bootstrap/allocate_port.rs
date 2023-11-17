use cgp_core::prelude::*;

/// Allocate a TCP port that the full node process use for listening
#[async_trait]
pub trait CanAllocateTcpPort: HasErrorType {
    async fn allocate_tcp_port(&self) -> Result<u16, Self::Error>;
}
