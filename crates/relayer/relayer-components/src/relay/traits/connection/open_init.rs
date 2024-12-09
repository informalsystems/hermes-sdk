use cgp::prelude::*;

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
        init_connection_options: &<Self::SrcChain as HasInitConnectionOptionsType<
            Self::DstChain,
        >>::InitConnectionOptions,
    ) -> Result<SrcConnectionId<Self>, Self::Error>;
}
