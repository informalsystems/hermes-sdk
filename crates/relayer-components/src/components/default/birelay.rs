use core::marker::PhantomData;

use cgp_core::prelude::*;
use cgp_core::RunnerComponent;

use crate::relay::components::auto_relayers::both_ways::RelayBothWays;

pub struct DefaultBiRelayComponents<BaseComponents>(pub PhantomData<BaseComponents>);

delegate_components!(
    DefaultBiRelayComponents<BaseComponents>;
    RunnerComponent: RelayBothWays
);
