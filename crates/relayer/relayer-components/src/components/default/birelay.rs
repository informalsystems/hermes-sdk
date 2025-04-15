#[cgp::re_export_imports]
mod preset {
    use cgp::extra::run::RunnerComponent;
    use cgp::prelude::*;

    use crate::birelay::impls::PerformAutoBiRelay;
    use crate::birelay::traits::AutoBiRelayerComponent;
    use crate::relay::impls::RelayBothWays;

    cgp_preset! {
        DefaultBiRelayComponents {
            RunnerComponent: RelayBothWays,
            AutoBiRelayerComponent: PerformAutoBiRelay,
        }
    }
}
