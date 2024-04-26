use cgp_core::CanRaiseError;
use hermes_relayer_components::chain::traits::queries::consensus_state::RawConsensusStateQuerier;
use hermes_relayer_components::chain::traits::types::consensus_state::HasRawConsensusStateType;
use hermes_relayer_components::chain::traits::types::height::HasHeightType;
use hermes_relayer_components::chain::traits::types::ibc::HasIbcChainTypes;
use ibc_relayer_types::core::ics24_host::identifier::ClientId;
use ibc_relayer_types::Height;
use prost::{DecodeError, Message};
use prost_types::Any;

use crate::traits::abci_query::CanQueryAbci;

pub struct QueryCosmosConsensusStateFromAbci;

pub const IBC_QUERY_PATH: &str = "store/ibc/key";

impl<Chain, Counterparty> RawConsensusStateQuerier<Chain, Counterparty>
    for QueryCosmosConsensusStateFromAbci
where
    Chain: HasIbcChainTypes<Counterparty, ClientId = ClientId, Height = Height>
        + HasRawConsensusStateType<RawConsensusState = Any>
        + CanQueryAbci
        + CanRaiseError<DecodeError>,
    Counterparty: HasHeightType<Height = Height>,
{
    async fn query_raw_consensus_state(
        chain: &Chain,
        client_id: &ClientId,
        consensus_height: &Height,
        query_height: &Height,
    ) -> Result<Any, Chain::Error> {
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

        let consensus_state_any =
            Message::decode(consensus_state_bytes.as_ref()).map_err(Chain::raise_error)?;

        Ok(consensus_state_any)
    }
}
