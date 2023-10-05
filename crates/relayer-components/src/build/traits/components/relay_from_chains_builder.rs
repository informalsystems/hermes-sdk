use cgp_core::prelude::*;
use cgp_core::HasErrorType;

use crate::build::traits::birelay::HasBiRelayType;
use crate::build::traits::target::relay::RelayBuildTarget;
use crate::build::types::aliases::{
    TargetDstChain, TargetDstClientId, TargetRelay, TargetSrcChain, TargetSrcClientId,
};
use crate::std_prelude::*;

#[derive_component(RelayFromChainsBuilderComponent, RelayFromChainsBuilder<Build>)]
#[async_trait]
pub trait CanBuildRelayFromChains<Target>: HasBiRelayType + HasErrorType
where
    Target: RelayBuildTarget<Self>,
{
    async fn build_relay_from_chains(
        &self,
        target: Target,
        src_client_id: &TargetSrcClientId<Self, Target>,
        dst_client_id: &TargetDstClientId<Self, Target>,
        src_chain: TargetSrcChain<Self, Target>,
        dst_chain: TargetDstChain<Self, Target>,
    ) -> Result<TargetRelay<Self, Target>, Self::Error>;
}
