use cgp_async::async_trait;
use cgp_macros::derive_component;

use crate::relay::traits::chains::HasRelayChains;
use crate::relay::types::aliases::{DstConnectionId, SrcConnectionId};
use crate::std_prelude::*;

#[derive_component(ConnectionOpenTryRelayerComponent, ConnectionOpenTryRelayer<Relay>)]
#[async_trait]
pub trait CanRelayConnectionOpenTry: HasRelayChains {
    async fn relay_connection_open_try(
        &self,
        src_connection_id: &SrcConnectionId<Self>,
    ) -> Result<DstConnectionId<Self>, Self::Error>;
}
