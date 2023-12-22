use alloc::boxed::Box;

use cgp_core::prelude::*;

#[derive_component(TcpPortReserverComponent, TcpPortReserver<Runtime>)]
/// Allocate a TCP port that the full node process use for listening
#[async_trait]
pub trait CanReserveTcpPort: HasErrorType {
    async fn reserve_tcp_port(&self) -> Result<u16, Self::Error>;
}
