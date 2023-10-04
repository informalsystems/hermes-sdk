use cgp_core::{async_trait, derive_component};

use crate::chain::traits::types::connection::HasInitConnectionOptionsType;
use crate::relay::traits::chains::HasRelayChains;
use crate::relay::types::aliases::SrcConnectionId;
use crate::std_prelude::*;

#[derive_component(ConnectionInitializerComponent, ConnectionInitializer<Relay>)]
#[async_trait]
pub trait CanInitConnection: HasRelayChains
where
    Self::SrcChain: HasInitConnectionOptionsType<Self::DstChain>,
{
    async fn init_connection(
        &self,
        init_connection_options: &<Self::SrcChain as HasInitConnectionOptionsType<
            Self::DstChain,
        >>::InitConnectionOptions,
    ) -> Result<SrcConnectionId<Self>, Self::Error>;
}
