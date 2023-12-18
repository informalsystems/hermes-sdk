use core::marker::PhantomData;

use cgp_core::prelude::*;
use cgp_core::ErrorRaiserComponent;
use cgp_core::ErrorTypeComponent;

use crate::build::components::birelay::BuildBiRelayFromRelays;
use crate::build::components::chain::cache::BuildChainWithCache;
use crate::build::components::relay::build_from_chain::BuildRelayFromChains;
use crate::build::components::relay::cache::BuildRelayWithCache;
use crate::build::traits::components::birelay_builder::BiRelayBuilderComponent;
use crate::build::traits::components::birelay_from_relay_builder::BiRelayFromRelayBuilderComponent;
use crate::build::traits::components::chain_builder::ChainBuilderComponent;
use crate::build::traits::components::relay_builder::RelayBuilderComponent;
use crate::build::traits::components::relay_from_chains_builder::RelayFromChainsBuilderComponent;
use crate::logger::traits::has_logger::{LoggerFieldComponent, LoggerTypeComponent};
use crate::runtime::traits::runtime::RuntimeComponent;
use crate::runtime::traits::runtime::RuntimeTypeComponent;

pub struct DefaultBuildComponents<BaseComponents>(pub PhantomData<BaseComponents>);

delegate_components!(
    DefaultBuildComponents<BaseComponents>;
    ChainBuilderComponent: BuildChainWithCache<BaseComponents>,
    RelayBuilderComponent: BuildRelayWithCache<BuildRelayFromChains>,
    BiRelayBuilderComponent: BuildBiRelayFromRelays,
    [
        ErrorTypeComponent,
        ErrorRaiserComponent,
        RuntimeTypeComponent,
        RuntimeComponent,
        LoggerFieldComponent,
        LoggerTypeComponent,
        RelayFromChainsBuilderComponent,
        BiRelayFromRelayBuilderComponent,
    ]: BaseComponents
);
