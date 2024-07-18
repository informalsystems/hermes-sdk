use cgp_core::prelude::*;
use hermes_relayer_components::birelay::traits::two_way::HasTwoWayRelay;
use hermes_relayer_components::build::traits::birelay::HasBiRelayType;
pub use hermes_relayer_components::build::traits::builders::birelay_builder::{
    BiRelayBuilderComponent, CanBuildBiRelay,
};
use hermes_relayer_components::build::traits::builders::birelay_from_relay_builder::BiRelayFromRelayBuilder;
pub use hermes_relayer_components::build::traits::builders::chain_builder::{
    ChainBuilder, ChainBuilderComponent,
};
pub use hermes_relayer_components::build::traits::builders::relay_builder::RelayBuilderComponent;
pub use hermes_relayer_components::build::traits::builders::relay_from_chains_builder::RelayFromChainsBuilderComponent;
use hermes_relayer_components::build::traits::cache::{HasChainCache, HasRelayCache};
use hermes_relayer_components::build::traits::target::chain::{ChainATarget, ChainBTarget};
use hermes_relayer_components::build::traits::target::relay::{RelayAToBTarget, RelayBToATarget};
use hermes_relayer_components::build::types::aliases::{ChainA, ChainB};
use hermes_relayer_components::chain::traits::types::chain_id::HasChainId;
use hermes_relayer_components::chain::traits::types::ibc::HasIbcChainTypes;
use hermes_relayer_components::components::default::build::DefaultBuildComponents;
use hermes_relayer_components::relay::traits::chains::{CanRaiseRelayChainErrors, HasRelayChains};
use hermes_runtime_components::traits::channel::{CanCloneSender, CanCreateChannels};
use hermes_runtime_components::traits::channel_once::CanUseChannelsOnce;
use hermes_runtime_components::traits::runtime::HasRuntime;

use crate::batch::traits::config::HasBatchConfig;
use crate::build::components::relay::batch::BuildRelayWithBatchWorker;
use crate::build::traits::cache::HasBatchSenderCache;
use crate::build::traits::components::relay_with_batch_builder::RelayWithBatchBuilder;
use crate::components::extra::closures::batch::UseBatchMessageWorkerSpawner;

define_components! {
    ExtraBuildComponents<BaseComponents: Async> {
        RelayFromChainsBuilderComponent: BuildRelayWithBatchWorker,
        [
            ChainBuilderComponent,
            RelayBuilderComponent,
            BiRelayBuilderComponent,
        ]:
            DefaultBuildComponents<BaseComponents>,
    }
}

pub trait CanUseExtraBuildComponents: UseExtraBuildComponents
where
    ChainA<Self>: HasIbcChainTypes<ChainB<Self>>,
    ChainB<Self>: HasIbcChainTypes<ChainA<Self>>,
{
}

pub trait UseExtraBuildComponents: CanBuildBiRelay
where
    ChainA<Self>: HasIbcChainTypes<ChainB<Self>>,
    ChainB<Self>: HasIbcChainTypes<ChainA<Self>>,
{
}

impl<Build, BiRelay, RelayAToB, RelayBToA, ChainA, ChainB, Error, Components, BaseComponents>
    UseExtraBuildComponents for Build
where
    Build: HasBatchConfig
        + HasBiRelayType<BiRelay = BiRelay>
        + HasRelayCache<RelayAToBTarget>
        + HasRelayCache<RelayBToATarget>
        + HasChainCache<ChainATarget>
        + HasChainCache<ChainBTarget>
        + HasBatchSenderCache<ChainATarget, Error>
        + HasBatchSenderCache<ChainBTarget, Error>
        + HasComponents<Components = Components>,
    BiRelay: HasTwoWayRelay<
        ChainA = ChainA,
        ChainB = ChainB,
        RelayAToB = RelayAToB,
        RelayBToA = RelayBToA,
    >,
    RelayAToB: Clone
        + HasErrorType<Error = Error>
        + HasRelayChains<SrcChain = ChainA, DstChain = ChainB>
        + UseBatchMessageWorkerSpawner
        + CanRaiseRelayChainErrors,
    RelayBToA: Clone
        + HasErrorType<Error = Error>
        + HasRelayChains<SrcChain = ChainB, DstChain = ChainA>
        + UseBatchMessageWorkerSpawner
        + CanRaiseRelayChainErrors,
    ChainA: Clone + HasErrorType + HasRuntime + HasChainId + HasIbcChainTypes<ChainB>,
    ChainB: Clone + HasErrorType + HasRuntime + HasChainId + HasIbcChainTypes<ChainA>,
    Error: Async,
    ChainA::ChainId: Ord + Clone,
    ChainB::ChainId: Ord + Clone,
    ChainA::ClientId: Ord + Clone,
    ChainB::ClientId: Ord + Clone,
    ChainA::Runtime: CanCreateChannels + CanUseChannelsOnce + CanCloneSender,
    ChainB::Runtime: CanCreateChannels + CanUseChannelsOnce + CanCloneSender,
    Components: HasComponents<Components = BaseComponents>
        + DelegatesToExtraBuildComponents<BaseComponents>
        + BiRelayFromRelayBuilder<Build>
        + RelayWithBatchBuilder<Build, RelayAToBTarget>
        + RelayWithBatchBuilder<Build, RelayBToATarget>,
    BaseComponents: ChainBuilder<Build, ChainATarget> + ChainBuilder<Build, ChainBTarget>,
{
}
