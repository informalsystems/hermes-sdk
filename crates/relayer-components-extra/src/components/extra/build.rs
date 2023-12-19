use core::marker::PhantomData;

use cgp_core::prelude::*;
use ibc_relayer_components::build::traits::components::birelay_builder::BiRelayBuilderComponent;
use ibc_relayer_components::build::traits::components::chain_builder::ChainBuilderComponent;
use ibc_relayer_components::build::traits::components::relay_builder::RelayBuilderComponent;
use ibc_relayer_components::build::traits::components::relay_from_chains_builder::RelayFromChainsBuilderComponent;
use ibc_relayer_components::components::default::build::DefaultBuildComponents;

use crate::build::components::relay::batch::BuildRelayWithBatchWorker;

pub struct ExtraBuildComponents<BaseComponents>(pub PhantomData<BaseComponents>);

delegate_components!(
    #[mark_component(IsExtraBuildComponent)]
    #[mark_delegate(DelegatesToExtraBuildComponents)]
    ExtraBuildComponents<BaseComponents>;

    RelayFromChainsBuilderComponent: BuildRelayWithBatchWorker,
    [
        ChainBuilderComponent,
        RelayBuilderComponent,
        BiRelayBuilderComponent,
    ]:
        DefaultBuildComponents<BaseComponents>,
);
