use core::marker::PhantomData;

use cgp::prelude::*;
use hermes_relayer_components::chain::traits::{
    ConsensusStateQuerier, ConsensusStateQuerierComponent, HasConsensusStateType, HasHeightType,
    HasIbcChainTypes,
};

use crate::telemetry::traits::metrics::{HasMetric, TelemetryCounter};
use crate::telemetry::traits::telemetry::HasTelemetry;

pub struct ConsensusStateTelemetryQuerier<InQuerier> {
    pub querier: InQuerier,
}

#[cgp_provider(ConsensusStateQuerierComponent)]
impl<InQuerier, Chain, Counterparty, Telemetry> ConsensusStateQuerier<Chain, Counterparty>
    for ConsensusStateTelemetryQuerier<InQuerier>
where
    Chain: HasIbcChainTypes<Counterparty> + HasTelemetry<Telemetry = Telemetry> + HasAsyncErrorType,
    Counterparty: HasConsensusStateType<Chain> + HasHeightType,
    InQuerier: ConsensusStateQuerier<Chain, Counterparty>,
    Telemetry: HasMetric<TelemetryCounter>,
    Telemetry::Value: From<u64>,
{
    async fn query_consensus_state(
        chain: &Chain,
        tag: PhantomData<Counterparty>,
        client_id: &Chain::ClientId,
        consensus_height: &Counterparty::Height,
        query_height: &Chain::Height,
    ) -> Result<Counterparty::ConsensusState, Chain::Error> {
        let telemetry = chain.telemetry();
        let label = Telemetry::new_label("query_type", "consensus_state");
        telemetry.update_metric("query", &[label], 1u64.into(), None, None);
        let status =
            InQuerier::query_consensus_state(chain, tag, client_id, consensus_height, query_height)
                .await?;
        Ok(status)
    }
}
