use hermes_cosmos_chain_components::types::ChainStatus;
use hermes_prelude::*;
use hermes_relayer_components::chain::traits::{
    ChainStatusQuerier, ChainStatusQuerierComponent, HasChainStatusType,
};
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
