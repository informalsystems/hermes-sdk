use hermes_chain_components::traits::InitConnectionOptionsOf;
use hermes_prelude::*;

use crate::chain::traits::HasInitConnectionOptionsType;
use crate::relay::traits::HasRelayChains;
use crate::relay::types::SrcConnectionId;

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
