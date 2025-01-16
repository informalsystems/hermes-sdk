use alloc::vec::Vec;
use core::marker::PhantomData;

use cgp::prelude::{Async, CanRaiseAsyncError};
use hermes_encoding_components::traits::decode::CanDecode;
use hermes_encoding_components::traits::has_encoding::HasDefaultEncoding;
use hermes_encoding_components::traits::types::encoded::HasEncodedType;
use hermes_encoding_components::types::AsBytes;

use crate::traits::queries::consensus_state::{CanQueryRawConsensusState, ConsensusStateQuerier};
use crate::traits::types::consensus_state::HasConsensusStateType;
use crate::traits::types::height::HasHeightType;

pub struct QueryAndDecodeConsensusState<Strategy>(pub PhantomData<Strategy>);

impl<Chain, Counterparty, Encoding, Strategy> ConsensusStateQuerier<Chain, Counterparty>
    for QueryAndDecodeConsensusState<Strategy>
where
    Chain: CanQueryRawConsensusState<Counterparty, RawConsensusState = Vec<u8>>
        + CanRaiseAsyncError<Encoding::Error>,
    Counterparty: HasConsensusStateType<Chain>
        + HasHeightType
        + HasDefaultEncoding<AsBytes, Encoding = Encoding>,
    Encoding: HasEncodedType<Encoded = Vec<u8>> + CanDecode<Strategy, Counterparty::ConsensusState>,
    Strategy: Async,
{
    async fn query_consensus_state(
        chain: &Chain,
        _tag: PhantomData<Counterparty>,
        client_id: &Chain::ClientId,
        consensus_height: &Counterparty::Height,
        query_height: &Chain::Height,
    ) -> Result<Counterparty::ConsensusState, Chain::Error> {
        let consensus_state_bytes = chain
            .query_raw_consensus_state(client_id, consensus_height, query_height)
            .await?;

        let consensus_state = Counterparty::default_encoding()
            .decode(&consensus_state_bytes)
            .map_err(Chain::raise_error)?;

        Ok(consensus_state)
    }
}
