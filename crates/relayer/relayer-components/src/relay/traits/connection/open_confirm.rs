use cgp::prelude::*;

use crate::relay::traits::chains::HasRelayChains;
use crate::relay::types::aliases::{DstConnectionId, SrcConnectionId};

#[cgp_component {
  provider: ConnectionOpenConfirmRelayer,
  context: Relay,
}]
#[async_trait]
pub trait CanRelayConnectionOpenConfirm: HasRelayChains {
    async fn relay_connection_open_confirm(
        &self,
        src_connection_id: &SrcConnectionId<Self>,
        dst_connection_id: &DstConnectionId<Self>,
    ) -> Result<(), Self::Error>;
}
