use async_trait::async_trait;
use ibc_relayer_components::chain::traits::components::consensus_state_querier::ConsensusStateQuerier;
use ibc_relayer_components::chain::traits::types::consensus_state::HasConsensusStateType;
use ibc_relayer_components::chain::traits::types::height::HasHeightType;
use ibc_relayer_cosmos::types::tendermint::TendermintConsensusState;
use ibc_relayer_types::core::ics24_host::identifier::ClientId;
use ibc_relayer_types::Height;

use crate::traits::solomachine::Solomachine;
use crate::types::chain::SolomachineChain;

pub struct QueryCosmosConsensusStateFromSolomachine;

#[async_trait]
impl<Chain, Counterparty> ConsensusStateQuerier<SolomachineChain<Chain>, Counterparty>
    for QueryCosmosConsensusStateFromSolomachine
where
    Chain: Solomachine,
    Counterparty: HasHeightType<Height = Height>
        + HasConsensusStateType<SolomachineChain<Chain>, ConsensusState = TendermintConsensusState>,
{
    async fn query_consensus_state(
        chain: &SolomachineChain<Chain>,
        client_id: &ClientId,
        height: &Height,
    ) -> Result<TendermintConsensusState, Chain::Error> {
        chain.chain.query_consensus_state(client_id, *height).await
    }
}
