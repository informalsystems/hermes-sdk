#[cgp::re_export_imports]
mod preset {
    use cgp::prelude::*;
    use hermes_relayer_components::chain::traits::queries::chain_status::ChainStatusQuerierComponent;
    use hermes_relayer_components::chain::traits::queries::consensus_state::ConsensusStateQuerierComponent;

    use crate::telemetry::components::consensus_state::ConsensusStateTelemetryQuerier;
    use crate::telemetry::components::status::ChainStatusTelemetryQuerier;

    cgp_preset! {
        ExtraChainComponents<BaseComponents: Async> {
            ChainStatusQuerierComponent:
                ChainStatusTelemetryQuerier<BaseComponents>,
            ConsensusStateQuerierComponent:
                ConsensusStateTelemetryQuerier<BaseComponents>,
        }
    }
}
