use cgp::prelude::*;
use hermes_relayer_components::chain::traits::queries::chain_status::*;
use hermes_relayer_components::chain::traits::types::status::HasChainStatusType;

use crate::telemetry::traits::metrics::{HasMetric, TelemetryCounter};
use crate::telemetry::traits::telemetry::HasTelemetry;

pub struct ChainStatusTelemetryQuerier<InQuerier> {
    pub querier: InQuerier,
}

#[cgp_provider(ChainStatusQuerierComponent)]
impl<InQuerier, Chain, Telemetry> ChainStatusQuerier<Chain>
    for ChainStatusTelemetryQuerier<InQuerier>
where
    InQuerier: ChainStatusQuerier<Chain>,
    Chain: HasChainStatusType + HasTelemetry<Telemetry = Telemetry> + HasAsyncErrorType,
    Telemetry: HasMetric<TelemetryCounter>,
    Telemetry::Value: From<u64>,
{
    async fn query_chain_status(context: &Chain) -> Result<Chain::ChainStatus, Chain::Error> {
        let telemetry = context.telemetry();
        let label = Telemetry::new_label("query_type", "status");
        telemetry.update_metric("query", &[label], 1u64.into(), None, None);
        let status = InQuerier::query_chain_status(context).await?;
        Ok(status)
    }
}
