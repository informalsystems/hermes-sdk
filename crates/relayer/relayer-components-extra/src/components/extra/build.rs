use cgp::core::error::ProvideErrorType;
use cgp::core::field::Index;
use cgp::prelude::*;
pub use hermes_relayer_components::build::traits::builders::birelay_builder::{
    BiRelayBuilderComponent, CanBuildBiRelay,
};
use hermes_relayer_components::build::traits::builders::birelay_from_relay_builder::BiRelayFromRelayBuilder;
pub use hermes_relayer_components::build::traits::builders::chain_builder::{
    ChainBuilder, ChainBuilderComponent,
};
pub use hermes_relayer_components::build::traits::builders::relay_builder::RelayBuilderComponent;
use hermes_relayer_components::build::traits::builders::relay_from_chains_builder::CanBuildRelayFromChains;
pub use hermes_relayer_components::build::traits::builders::relay_from_chains_builder::RelayFromChainsBuilderComponent;
use hermes_relayer_components::build::traits::cache::{HasChainCache, HasRelayCache};
use hermes_relayer_components::chain::traits::types::chain_id::HasChainId;
use hermes_relayer_components::chain::traits::types::ibc::HasIbcChainTypes;
use hermes_relayer_components::components::default::build::DefaultBuildComponents;
use hermes_relayer_components::multi::traits::birelay_at::HasBiRelayTypeAt;
use hermes_relayer_components::multi::traits::chain_at::HasChainTypeAt;
use hermes_relayer_components::multi::traits::relay_at::HasRelayTypeAt;
use hermes_relayer_components::relay::traits::chains::{
    CanRaiseRelayChainErrors, HasRelayChains, HasRelayClientIds,
};
use hermes_relayer_components::relay::traits::target::{DestinationTarget, SourceTarget};
use hermes_runtime_components::traits::channel::{CanCloneSender, CanCreateChannels};
use hermes_runtime_components::traits::channel_once::CanUseChannelsOnce;
use hermes_runtime_components::traits::runtime::{HasRuntime, HasRuntimeType};

use crate::batch::traits::config::HasBatchConfig;
use crate::build::impls::relay::batch::BuildRelayWithBatchWorker;
use crate::build::traits::cache::HasBatchSenderCache;
use crate::build::traits::relay_with_batch_builder::RelayWithBatchBuilder;

cgp_preset! {
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
