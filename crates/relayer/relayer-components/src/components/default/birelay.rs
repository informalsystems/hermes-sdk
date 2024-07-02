use cgp_core::prelude::*;
pub use cgp_core::run::RunnerComponent;

use crate::relay::impls::auto_relayers::both_ways::RelayBothWays;

define_components! {
    DefaultBiRelayComponents {
        RunnerComponent: RelayBothWays,
    }
}
