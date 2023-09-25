use core::marker::PhantomData;

use cgp_core::delegate_component;
use ibc_relayer_components::relay::traits::components::auto_relayer::AutoRelayerComponent;

use crate::relay::components::auto_relayers::parallel_two_way::ParallelTwoWayAutoRelay;
pub struct ExtraBiRelayComponents<BaseComponents>(pub PhantomData<BaseComponents>);

delegate_component!(
    AutoRelayerComponent,
    ExtraBiRelayComponents<BaseComponents>,
    ParallelTwoWayAutoRelay,
);
