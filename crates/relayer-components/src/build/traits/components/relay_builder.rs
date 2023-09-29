use cgp_async::async_generic_trait;
use cgp_core::traits::HasErrorType;
use cgp_macros::derive_component;

use crate::build::traits::birelay::HasBiRelayType;
use crate::build::traits::target::relay::RelayBuildTarget;
use crate::build::types::aliases::{
    TargetDstChainId, TargetDstClientId, TargetRelay, TargetSrcChainId, TargetSrcClientId,
};
use crate::std_prelude::*;

#[derive_component(RelayBuilderComponent, RelayBuilder<Build>)]
#[async_generic_trait]
pub trait CanBuildRelay<Target>: HasBiRelayType + HasErrorType
where
    Target: RelayBuildTarget<Self>,
{
    async fn build_relay(
        &self,
        target: Target,
        src_chain_id: &TargetSrcChainId<Self, Target>,
        dst_chain_id: &TargetDstChainId<Self, Target>,
        src_client_id: &TargetSrcClientId<Self, Target>,
        dst_client_id: &TargetDstClientId<Self, Target>,
    ) -> Result<TargetRelay<Self, Target>, Self::Error>;
}
