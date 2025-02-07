use cgp::prelude::*;
use hermes_cosmos_chain_components::types::status::ChainStatus;
use hermes_cosmos_relayer::presets::chain::ChainStatusQuerierComponent;
use hermes_relayer_components::chain::traits::queries::chain_status::ChainStatusQuerier;
use hermes_relayer_components::chain::traits::types::status::HasChainStatusType;
use ibc::core::client::types::Height;
use tendermint::Time;

pub struct QuerySolomachineStatus;

#[cgp_provider(ChainStatusQuerierComponent)]
impl<Chain> ChainStatusQuerier<Chain> for QuerySolomachineStatus
where
    Chain: HasChainStatusType<ChainStatus = ChainStatus> + HasAsyncErrorType,
{
    async fn query_chain_status(_chain: &Chain) -> Result<ChainStatus, Chain::Error> {
        // stub
        Ok(ChainStatus {
            height: Height::new(0, 1).unwrap(),
            time: Time::now(),
        })
    }
}
