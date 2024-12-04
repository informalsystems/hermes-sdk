use cgp::core::error::ProvideErrorType;
use cgp::prelude::*;
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
use hermes_relayer_components::chain::traits::types::chain_id::HasChainId;
use hermes_relayer_components::chain::traits::types::ibc::HasIbcChainTypes;
use hermes_relayer_components::components::default::build::DefaultBuildComponents;
use hermes_relayer_components::multi::traits::birelay_at::HasBiRelayTypeAt;
use hermes_relayer_components::multi::traits::chain_at::HasChainTypeAt;
use hermes_relayer_components::multi::traits::relay_at::HasRelayTypeAt;
use hermes_relayer_components::multi::types::index::Index;
use hermes_relayer_components::relay::traits::chains::{
    CanRaiseRelayChainErrors, HasRelayChains, HasRelayClientIds,
};
use hermes_runtime_components::traits::channel::{CanCloneSender, CanCreateChannels};
use hermes_runtime_components::traits::channel_once::CanUseChannelsOnce;
use hermes_runtime_components::traits::runtime::HasRuntime;

use crate::batch::traits::config::HasBatchConfig;
use crate::build::impls::relay::batch::BuildRelayWithBatchWorker;
use crate::build::traits::cache::HasBatchSenderCache;
use crate::build::traits::relay_with_batch_builder::RelayWithBatchBuilder;
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

pub trait CanUseExtraBuildComponents: UseExtraBuildComponents {}

pub trait UseExtraBuildComponents: CanBuildBiRelay<Index<0>, Index<1>> {}

impl<Build, RelayAToB, RelayBToA, ChainA, ChainB, Error, Components, BaseComponents>
    UseExtraBuildComponents for Build
where
    Build: HasBatchConfig
        + HasBiRelayTypeAt<Index<0>, Index<1>>
        + HasRelayTypeAt<Index<0>, Index<1>, Relay = RelayAToB>
        + HasRelayTypeAt<Index<1>, Index<0>, Relay = RelayBToA>
        + HasChainTypeAt<Index<0>, Chain = ChainA>
        + HasChainTypeAt<Index<1>, Chain = ChainB>
        + HasRelayCache<Index<0>, Index<1>>
        + HasRelayCache<Index<1>, Index<0>>
        + HasChainCache<Index<0>>
        + HasChainCache<Index<1>>
        + HasBatchSenderCache<Error, Index<0>, Index<1>>
        + HasBatchSenderCache<Error, Index<1>, Index<0>>
        + HasComponents<Components = Components>,
    RelayAToB: Clone
        + HasErrorType<Error = Error>
        + HasRelayChains<SrcChain = ChainA, DstChain = ChainB>
        + HasRelayClientIds
        + UseBatchMessageWorkerSpawner
        + CanRaiseRelayChainErrors,
    RelayBToA: Clone
        + HasErrorType<Error = Error>
        + HasRelayChains<SrcChain = ChainB, DstChain = ChainA>
        + HasRelayClientIds
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
        + BiRelayFromRelayBuilder<Build, Index<0>, Index<1>>
        + RelayWithBatchBuilder<Build, Index<0>, Index<1>>
        + RelayWithBatchBuilder<Build, Index<1>, Index<0>>
        + ProvideErrorType<Build>,
    BaseComponents: Async + ChainBuilder<Build, Index<0>> + ChainBuilder<Build, Index<1>>,
{
}
