use core::marker::PhantomData;

use cgp_core::prelude::*;
use hermes_relayer_components::chain::traits::queries::chain_status::ChainStatusQuerierComponent;
use hermes_relayer_components::chain::traits::queries::consensus_state::ConsensusStateQuerierComponent;

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
