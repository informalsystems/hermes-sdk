use core::marker::PhantomData;

use cgp_core::prelude::*;
use cgp_core::RunnerComponent;
use ibc_relayer_components::components::default::birelay::DefaultBiRelayComponents;

pub struct ExtraBiRelayComponents<BaseComponents>(pub PhantomData<BaseComponents>);

delegate_components!(
    ExtraBiRelayComponents<BaseComponents>;
    RunnerComponent:
        DefaultBiRelayComponents<BaseComponents>,
);
