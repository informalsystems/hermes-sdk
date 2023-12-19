use cgp_core::prelude::*;
use cgp_core::RunnerComponent;

use crate::relay::components::auto_relayers::both_ways::RelayBothWays;

pub struct DefaultBiRelayComponents;

delegate_components!(
    #[mark_component(IsDefaultBiRelayComponent)]
    #[mark_delegate(DelegatesToDefaultBiRelayComponents)]
    DefaultBiRelayComponents {
        RunnerComponent: RelayBothWays,
    }
);
