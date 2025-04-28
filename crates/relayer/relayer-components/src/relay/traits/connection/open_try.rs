use cgp::prelude::*;

use crate::relay::traits::HasRelayChains;
use crate::relay::types::{DstConnectionId, SrcConnectionId};

#[cgp_component {
  provider: ConnectionOpenTryRelayer,
  context: Relay,
}]
#[async_trait]
pub trait CanRelayConnectionOpenTry: HasRelayChains {
    async fn relay_connection_open_try(
        &self,
        src_connection_id: &SrcConnectionId<Self>,
    ) -> Result<DstConnectionId<Self>, Self::Error>;
}
