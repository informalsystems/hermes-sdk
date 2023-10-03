use cgp_core::{async_trait, derive_component};

use crate::relay::traits::chains::HasRelayChains;
use crate::relay::types::aliases::{DstConnectionId, SrcConnectionId};
use crate::std_prelude::*;

#[derive_component(ConnectionOpenHandshakeRelayerComponent, ConnectionOpenHandshakeRelayer<Relay>)]
#[async_trait]
pub trait CanRelayConnectionOpenHandshake: HasRelayChains {
    async fn relay_connection_open_handshake(
        &self,
        src_connection_id: &SrcConnectionId<Self>,
    ) -> Result<DstConnectionId<Self>, Self::Error>;
}
