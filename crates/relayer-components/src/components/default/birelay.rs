use core::marker::PhantomData;

use cgp_core::delegate_component;
use cgp_core::RunnerComponent;

use crate::relay::components::auto_relayers::both_ways::RelayBothWays;

pub struct DefaultBiRelayComponents<BaseComponents>(pub PhantomData<BaseComponents>);

delegate_component!(
    RunnerComponent,
    DefaultBiRelayComponents<BaseComponents>,
    RelayBothWays,
);
