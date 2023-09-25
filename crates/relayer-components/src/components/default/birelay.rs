use core::marker::PhantomData;

use cgp_core::delegate_component;

use crate::relay::components::auto_relayers::concurrent_two_way::ConcurrentTwoWayAutoRelay;
use crate::relay::traits::components::auto_relayer::AutoRelayerComponent;

pub struct DefaultBiRelayComponents<BaseComponents>(pub PhantomData<BaseComponents>);

delegate_component!(
    AutoRelayerComponent,
    DefaultBiRelayComponents<BaseComponents>,
    ConcurrentTwoWayAutoRelay,
);
