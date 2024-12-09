use cgp::prelude::*;

use crate::relay::traits::chains::HasRelayChains;
use crate::relay::types::aliases::{DstConnectionId, SrcConnectionId};

#[cgp_component {
  provider: ConnectionOpenHandshakeRelayer,
  context: Relay,
}]
#[async_trait]
pub trait CanRelayConnectionOpenHandshake: HasRelayChains {
    async fn relay_connection_open_handshake(
        &self,
        src_connection_id: &SrcConnectionId<Self>,
    ) -> Result<DstConnectionId<Self>, Self::Error>;
}
