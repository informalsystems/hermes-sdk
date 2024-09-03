use cgp::core::error::HasErrorType;
use hermes_relayer_components::chain::traits::queries::chain_status::ChainStatusQuerier;
use hermes_relayer_components::chain::traits::types::status::HasChainStatusType;
use ibc_relayer::chain::endpoint::ChainStatus;
use ibc_relayer_types::Height;

pub struct QuerySolomachineStatus;

impl<Chain> ChainStatusQuerier<Chain> for QuerySolomachineStatus
where
    Chain: HasChainStatusType<ChainStatus = ChainStatus> + HasErrorType,
{
    async fn query_chain_status(_chain: &Chain) -> Result<ChainStatus, Chain::Error> {
        // stub
        Ok(ChainStatus {
            height: Height::new(0, 1).unwrap(),
            timestamp: Default::default(),
        })
    }
}
