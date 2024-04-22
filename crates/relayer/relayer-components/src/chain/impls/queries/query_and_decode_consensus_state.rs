use alloc::vec::Vec;
use core::marker::PhantomData;

use cgp_core::{Async, CanRaiseError};
use hermes_encoding_components::traits::decoder::CanDecode;
use hermes_encoding_components::traits::encoded::HasEncodedType;
use hermes_encoding_components::traits::has_encoding::HasDefaultEncoding;

use crate::chain::traits::queries::consensus_state::{
    CanQueryConsensusStateBytes, ConsensusStateQuerier,
};
use crate::chain::traits::types::consensus_state::HasConsensusStateType;
use crate::chain::traits::types::height::HasHeightType;

pub struct QueryAndDecodeConsensusState<Strategy>(pub PhantomData<Strategy>);

impl<Chain, Counterparty, Encoding, Strategy> ConsensusStateQuerier<Chain, Counterparty>
    for QueryAndDecodeConsensusState<Strategy>
where
    Chain: CanQueryConsensusStateBytes<Counterparty> + CanRaiseError<Encoding::Error>,
    Counterparty:
        HasConsensusStateType<Chain> + HasHeightType + HasDefaultEncoding<Encoding = Encoding>,
    Encoding: HasEncodedType<Encoded = Vec<u8>> + CanDecode<Strategy, Counterparty::ConsensusState>,
    Strategy: Async,
{
    async fn query_consensus_state(
        chain: &Chain,
        client_id: &Chain::ClientId,
        consensus_height: &Counterparty::Height,
        query_height: &Chain::Height,
    ) -> Result<Counterparty::ConsensusState, Chain::Error> {
        let consensus_state_bytes = chain
            .query_consensus_state_bytes(client_id, consensus_height, query_height)
            .await?;

        let consensus_state = Counterparty::default_encoding()
            .decode(&consensus_state_bytes)
            .map_err(Chain::raise_error)?;

        Ok(consensus_state)
    }
}
