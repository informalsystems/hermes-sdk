use cgp_core::delegate_component;
use core::marker::PhantomData;

use crate::relay::components::auto_relayers::parallel_two_way::ParallelTwoWayAutoRelay;
use ibc_relayer_components::relay::traits::components::auto_relayer::AutoRelayerComponent;
pub struct ExtraBiRelayComponents<BaseComponents>(pub PhantomData<BaseComponents>);

delegate_component!(
    AutoRelayerComponent,
    ExtraBiRelayComponents<BaseComponents>,
    ParallelTwoWayAutoRelay,
);
