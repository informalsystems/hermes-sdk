use core::marker::PhantomData;

use cgp_core::delegate_component;
use ibc_relayer_components::core::traits::run::RunnerComponent;

use crate::relay::components::auto_relayers::parallel_two_way::ParallelTwoWayAutoRelay;
pub struct ExtraBiRelayComponents<BaseComponents>(pub PhantomData<BaseComponents>);

delegate_component!(
    RunnerComponent,
    ExtraBiRelayComponents<BaseComponents>,
    ParallelTwoWayAutoRelay,
);
