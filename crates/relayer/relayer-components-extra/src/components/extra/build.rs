use cgp::prelude::*;
pub use hermes_relayer_components::build::traits::builders::birelay_builder::{
    BiRelayBuilderComponent, CanBuildBiRelay,
};
pub use hermes_relayer_components::build::traits::builders::chain_builder::{
    ChainBuilder, ChainBuilderComponent,
};
pub use hermes_relayer_components::build::traits::builders::relay_builder::RelayBuilderComponent;
pub use hermes_relayer_components::build::traits::builders::relay_from_chains_builder::RelayFromChainsBuilderComponent;
use hermes_relayer_components::components::default::build::DefaultBuildComponents;

use crate::build::impls::relay::batch::BuildRelayWithBatchWorker;

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
