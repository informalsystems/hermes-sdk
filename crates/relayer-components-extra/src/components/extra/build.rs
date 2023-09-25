use core::marker::PhantomData;

use cgp_core::{delegate_component, delegate_components};
use ibc_relayer_components::build::traits::components::birelay_builder::BiRelayBuilderComponent;
use ibc_relayer_components::build::traits::components::birelay_from_relay_builder::BiRelayFromRelayBuilderComponent;
use ibc_relayer_components::build::traits::components::chain_builder::ChainBuilderComponent;
use ibc_relayer_components::build::traits::components::relay_builder::RelayBuilderComponent;
use ibc_relayer_components::build::traits::components::relay_from_chains_builder::RelayFromChainsBuilderComponent;
use ibc_relayer_components::components::default::build::DefaultBuildComponents;

use crate::build::components::relay::batch::BuildRelayWithBatchWorker;
use crate::build::traits::components::relay_with_batch_builder::RelayWithBatchBuilderComponent;

pub struct ExtraBuildComponents<BaseComponents>(pub PhantomData<BaseComponents>);

delegate_component!(
    RelayFromChainsBuilderComponent,
    ExtraBuildComponents<BaseComponents>,
    BuildRelayWithBatchWorker,
);

delegate_component!(
    RelayWithBatchBuilderComponent,
    ExtraBuildComponents<BaseComponents>,
    BaseComponents,
);

delegate_components!(
    [
        ChainBuilderComponent,
        RelayBuilderComponent,
        BiRelayBuilderComponent,
        BiRelayFromRelayBuilderComponent,
    ],
    ExtraBuildComponents<BaseComponents>,
    DefaultBuildComponents<BaseComponents>,
);
