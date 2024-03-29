use cgp_core::prelude::*;
use cgp_core::CanRaiseError;
use eyre::eyre;
use hermes_relayer_components::chain::traits::queries::consensus_state::ConsensusStateQuerier;
use hermes_relayer_components::chain::traits::types::consensus_state::HasConsensusStateType;
use hermes_relayer_components::chain::traits::types::height::HasHeightType;
use hermes_relayer_components::chain::traits::types::ibc::HasIbcChainTypes;
use ibc_relayer::chain::handle::ChainHandle;
use ibc_relayer::chain::requests::{IncludeProof, QueryConsensusStateRequest, QueryHeight};
use ibc_relayer::consensus_state::AnyConsensusState;
use ibc_relayer_types::core::ics24_host::identifier::ClientId;
use ibc_relayer_types::Height;

use crate::traits::chain_handle::HasBlockingChainHandle;
use crate::types::tendermint::TendermintConsensusState;

pub struct QueryCosmosConsensusStateFromChainHandle;

#[async_trait]
impl<Chain, Counterparty> ConsensusStateQuerier<Chain, Counterparty>
    for QueryCosmosConsensusStateFromChainHandle
where
    Chain: HasIbcChainTypes<Counterparty, ClientId = ClientId>
        + HasHeightType<Height = Height>
        + HasBlockingChainHandle
        + CanRaiseError<eyre::Report>,
    Counterparty: HasConsensusStateType<Chain, ConsensusState = TendermintConsensusState>
        + HasHeightType<Height = Height>,
{
    async fn query_consensus_state(
        chain: &Chain,
        client_id: &ClientId,
        consensus_height: &Height,
        query_height: &Height,
    ) -> Result<TendermintConsensusState, Chain::Error> {
        let client_id = client_id.clone();
        let consensus_height = *consensus_height;
        let query_height = QueryHeight::Specific(*query_height);

        chain
            .with_blocking_chain_handle(move |chain_handle| {
                let (any_consensus_state, _) = chain_handle
                    .query_consensus_state(
                        QueryConsensusStateRequest {
                            client_id: client_id.clone(),
                            consensus_height,
                            query_height,
                        },
                        IncludeProof::No,
                    )
                    .map_err(Chain::raise_error)?;

                match any_consensus_state {
                    AnyConsensusState::Tendermint(consensus_state) => Ok(consensus_state),
                    _ => Err(Chain::raise_error(eyre!(
                        "expected tendermint consensus state"
                    ))),
                }
            })
            .await
    }
}
