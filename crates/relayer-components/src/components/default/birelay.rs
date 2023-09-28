use core::marker::PhantomData;

use cgp_core::delegate_component;

use crate::core::traits::run::RunnerComponent;
use crate::relay::components::auto_relayers::concurrent_two_way::ConcurrentTwoWayAutoRelay;

pub struct DefaultBiRelayComponents<BaseComponents>(pub PhantomData<BaseComponents>);

delegate_component!(
    RunnerComponent,
    DefaultBiRelayComponents<BaseComponents>,
    ConcurrentTwoWayAutoRelay,
);
