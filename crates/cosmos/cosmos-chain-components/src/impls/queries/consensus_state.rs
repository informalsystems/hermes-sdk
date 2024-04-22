use hermes_relayer_components::chain::traits::queries::consensus_state::ConsensusStateBytesQuerier;
use hermes_relayer_components::chain::traits::types::height::HasHeightType;
use hermes_relayer_components::chain::traits::types::ibc::HasIbcChainTypes;
use ibc_relayer_types::core::ics24_host::identifier::ClientId;
use ibc_relayer_types::Height;

use crate::traits::abci_query::CanQueryAbci;

pub struct QueryCosmosConsensusStateFromAbci;

pub const IBC_QUERY_PATH: &str = "store/ibc/key";

impl<Chain, Counterparty> ConsensusStateBytesQuerier<Chain, Counterparty>
    for QueryCosmosConsensusStateFromAbci
where
    Chain: HasIbcChainTypes<Counterparty, ClientId = ClientId, Height = Height> + CanQueryAbci,
    Counterparty: HasHeightType<Height = Height>,
{
    async fn query_consensus_state_bytes(
        chain: &Chain,
        client_id: &ClientId,
        consensus_height: &Height,
        query_height: &Height,
    ) -> Result<Vec<u8>, Chain::Error> {
        let revision_number = consensus_height.revision_number();
        let revision_height = consensus_height.revision_height();
        let consensus_state_path =
            format!("clients/{client_id}/consensusStates/{revision_number}-{revision_height}");

        let consensus_state_bytes = chain
            .query_abci(
                IBC_QUERY_PATH,
                consensus_state_path.as_bytes(),
                query_height,
            )
            .await?;

        Ok(consensus_state_bytes)
    }
}
