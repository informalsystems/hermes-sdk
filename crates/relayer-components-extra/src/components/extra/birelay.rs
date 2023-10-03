use core::marker::PhantomData;

use cgp_core::delegate_component;
use ibc_relayer_components::components::default::birelay::DefaultBiRelayComponents;
use ibc_relayer_components::core::traits::run::RunnerComponent;

pub struct ExtraBiRelayComponents<BaseComponents>(pub PhantomData<BaseComponents>);

delegate_component!(
    RunnerComponent,
    ExtraBiRelayComponents<BaseComponents>,
    DefaultBiRelayComponents<BaseComponents>,
);
