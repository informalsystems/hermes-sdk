use core::marker::PhantomData;

use cgp_core::prelude::*;
use cgp_core::ErrorRaiserComponent;
use cgp_core::ErrorTypeComponent;
use ibc_relayer_components::build::traits::components::birelay_builder::BiRelayBuilderComponent;
use ibc_relayer_components::build::traits::components::birelay_from_relay_builder::BiRelayFromRelayBuilderComponent;
use ibc_relayer_components::build::traits::components::chain_builder::ChainBuilderComponent;
use ibc_relayer_components::build::traits::components::relay_builder::RelayBuilderComponent;
use ibc_relayer_components::build::traits::components::relay_from_chains_builder::RelayFromChainsBuilderComponent;
use ibc_relayer_components::components::default::build::DefaultBuildComponents;
use ibc_relayer_components::logger::traits::has_logger::{
    LoggerFieldComponent, LoggerTypeComponent,
};
use ibc_relayer_components::runtime::traits::runtime::RuntimeComponent;
use ibc_relayer_components::runtime::traits::runtime::RuntimeTypeComponent;

use crate::build::components::relay::batch::BuildRelayWithBatchWorker;
use crate::build::traits::components::relay_with_batch_builder::RelayWithBatchBuilderComponent;

pub struct ExtraBuildComponents<BaseComponents>(pub PhantomData<BaseComponents>);

delegate_components!(
    ExtraBuildComponents<BaseComponents>;
    RelayFromChainsBuilderComponent: BuildRelayWithBatchWorker,
    RelayWithBatchBuilderComponent: BaseComponents,
    [
        ErrorTypeComponent,
        ErrorRaiserComponent,
        RuntimeTypeComponent,
        RuntimeComponent,
        LoggerFieldComponent,
        LoggerTypeComponent,
        ChainBuilderComponent,
        RelayBuilderComponent,
        BiRelayBuilderComponent,
        BiRelayFromRelayBuilderComponent,
    ]:
        DefaultBuildComponents<BaseComponents>,
);
