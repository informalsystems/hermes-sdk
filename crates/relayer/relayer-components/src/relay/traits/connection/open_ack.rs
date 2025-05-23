use hermes_prelude::*;

use crate::relay::traits::HasRelayChains;
use crate::relay::types::{DstConnectionId, SrcConnectionId};

#[cgp_component {
  provider: ConnectionOpenAckRelayer,
  context: Relay,
}]
#[async_trait]
pub trait CanRelayConnectionOpenAck: HasRelayChains {
    async fn relay_connection_open_ack(
        &self,
        src_connection_id: &SrcConnectionId<Self>,
        dst_connection_id: &DstConnectionId<Self>,
    ) -> Result<(), Self::Error>;
}
