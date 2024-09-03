pub use cgp::extra::run::RunnerComponent;
use cgp_core::prelude::*;

use crate::relay::impls::auto_relayers::both_ways::RelayBothWays;

define_components! {
    DefaultBiRelayComponents {
        RunnerComponent: RelayBothWays,
    }
}
