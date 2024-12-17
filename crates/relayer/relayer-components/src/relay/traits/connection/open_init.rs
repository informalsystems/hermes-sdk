use cgp::prelude::*;
use hermes_chain_components::traits::types::connection::InitConnectionOptionsOf;

use crate::chain::traits::types::connection::HasInitConnectionOptionsType;
use crate::relay::traits::chains::HasRelayChains;
use crate::relay::types::aliases::SrcConnectionId;

#[cgp_component {
  provider: ConnectionInitializer,
  context: Relay,
}]
#[async_trait]
pub trait CanInitConnection:
    HasRelayChains<SrcChain: HasInitConnectionOptionsType<Self::DstChain>>
{
    async fn init_connection(
        &self,
        init_connection_options: &InitConnectionOptionsOf<Self::SrcChain, Self::DstChain>,
    ) -> Result<SrcConnectionId<Self>, Self::Error>;
}
