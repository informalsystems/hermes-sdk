use cgp_core::prelude::*;
use hermes_relayer_components::chain::traits::queries::chain_status::ChainStatusQuerierComponent;
use hermes_relayer_components::chain::traits::queries::consensus_state::ConsensusStateQuerierComponent;

use crate::telemetry::components::consensus_state::ConsensusStateTelemetryQuerier;
use crate::telemetry::components::status::ChainStatusTelemetryQuerier;

define_components! {
    ExtraChainComponents<BaseComponents: Async> {
        ChainStatusQuerierComponent:
            ChainStatusTelemetryQuerier<BaseComponents>,
        ConsensusStateQuerierComponent:
            ConsensusStateTelemetryQuerier<BaseComponents>,
    }
}
