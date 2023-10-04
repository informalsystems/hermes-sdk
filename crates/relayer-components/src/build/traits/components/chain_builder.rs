use cgp_core::prelude::*;
use cgp_core::HasErrorType;

use crate::build::traits::birelay::HasBiRelayType;
use crate::build::traits::target::chain::ChainBuildTarget;
use crate::build::types::aliases::{TargetChain, TargetChainId};
use crate::std_prelude::*;

#[derive_component(ChainBuilderComponent, ChainBuilder<Build>)]
#[async_trait]
pub trait CanBuildChain<Target>: HasBiRelayType + HasErrorType
where
    Target: ChainBuildTarget<Self>,
{
    async fn build_chain(
        &self,
        _target: Target,
        chain_id: &TargetChainId<Self, Target>,
    ) -> Result<TargetChain<Self, Target>, Self::Error>;
}
