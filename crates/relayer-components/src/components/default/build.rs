use core::marker::PhantomData;

use cgp_core::{delegate_component, delegate_components};

use crate::build::components::birelay::BuildBiRelayFromRelays;
use crate::build::components::chain::cache::BuildChainWithCache;
use crate::build::components::relay::build_from_chain::BuildRelayFromChains;
use crate::build::components::relay::cache::BuildRelayWithCache;
use crate::build::traits::components::birelay_builder::BiRelayBuilderComponent;
use crate::build::traits::components::birelay_from_relay_builder::BiRelayFromRelayBuilderComponent;
use crate::build::traits::components::chain_builder::ChainBuilderComponent;
use crate::build::traits::components::relay_builder::RelayBuilderComponent;
use crate::build::traits::components::relay_from_chains_builder::RelayFromChainsBuilderComponent;
pub struct DefaultBuildComponents<BaseComponents>(pub PhantomData<BaseComponents>);

delegate_component!(
    ChainBuilderComponent,
    DefaultBuildComponents<BaseComponents>,
    BuildChainWithCache<BaseComponents>,
);

delegate_component!(
    RelayBuilderComponent,
    DefaultBuildComponents<BaseComponents>,
    BuildRelayWithCache<BuildRelayFromChains>,
);

delegate_component!(
    BiRelayBuilderComponent,
    DefaultBuildComponents<BaseComponents>,
    BuildBiRelayFromRelays,
);

delegate_components!(
    [
        RelayFromChainsBuilderComponent,
        BiRelayFromRelayBuilderComponent,
    ],
    DefaultBuildComponents<BaseComponents>,
    BaseComponents,
);
