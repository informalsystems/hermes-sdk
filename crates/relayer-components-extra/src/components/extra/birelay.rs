use core::marker::PhantomData;

use cgp_core::delegate_component;
use cgp_core::RunnerComponent;
use ibc_relayer_components::components::default::birelay::DefaultBiRelayComponents;

pub struct ExtraBiRelayComponents<BaseComponents>(pub PhantomData<BaseComponents>);

delegate_component!(
    RunnerComponent,
    ExtraBiRelayComponents<BaseComponents>,
    DefaultBiRelayComponents<BaseComponents>,
);
