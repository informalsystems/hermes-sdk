use core::marker::PhantomData;

use cgp_core::prelude::*;
use hermes_relayer_components::chain::traits::components::chain_status_querier::ChainStatusQuerierComponent;
use hermes_relayer_components::chain::traits::components::consensus_state_querier::ConsensusStateQuerierComponent;

use crate::telemetry::components::consensus_state::ConsensusStateTelemetryQuerier;
use crate::telemetry::components::status::ChainStatusTelemetryQuerier;

pub struct ExtraChainComponents<BaseComponents>(pub PhantomData<BaseComponents>);

delegate_components! {
    #[mark_component(IsExtraChainComponent)]
    #[mark_delegate(DelegatesToExtraChainComponents)]
    ExtraChainComponents<BaseComponents> {
        ChainStatusQuerierComponent:
            ChainStatusTelemetryQuerier<BaseComponents>,
        ConsensusStateQuerierComponent:
            ConsensusStateTelemetryQuerier<BaseComponents>,
    }
}
