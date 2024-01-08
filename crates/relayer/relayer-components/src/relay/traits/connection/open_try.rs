use cgp_core::prelude::*;

use crate::relay::traits::chains::HasRelayChains;
use crate::relay::types::aliases::{DstConnectionId, SrcConnectionId};

#[derive_component(ConnectionOpenTryRelayerComponent, ConnectionOpenTryRelayer<Relay>)]
#[async_trait]
pub trait CanRelayConnectionOpenTry: HasRelayChains {
    async fn relay_connection_open_try(
        &self,
        src_connection_id: &SrcConnectionId<Self>,
    ) -> Result<DstConnectionId<Self>, Self::Error>;
}
