use hermes_cosmos_chain_components::types::tendermint::TendermintConsensusState;
use hermes_relayer_components::chain::traits::queries::consensus_state::ConsensusStateQuerier;
use hermes_relayer_components::chain::traits::types::consensus_state::HasConsensusStateType;
use hermes_relayer_components::chain::traits::types::height::HasHeightType;
use hermes_relayer_components::chain::traits::types::ibc::HasIbcChainTypes;

use crate::traits::solomachine::Solomachine;

pub struct QueryCosmosConsensusStateFromSolomachine;

impl<Chain, Counterparty> ConsensusStateQuerier<Chain, Counterparty>
    for QueryCosmosConsensusStateFromSolomachine
where
    Chain: Solomachine + HasIbcChainTypes<Counterparty, Height = Height, ClientId = ClientId>,
    Counterparty: HasHeightType<Height = Height>
        + HasConsensusStateType<Chain, ConsensusState = TendermintConsensusState>,
{
    async fn query_consensus_state(
        chain: &Chain,
        client_id: &ClientId,
        consensus_height: &Height,
        _query_height: &Height,
    ) -> Result<TendermintConsensusState, Chain::Error> {
        chain
            .query_consensus_state(client_id, *consensus_height)
            .await
    }
}
