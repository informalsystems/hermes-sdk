use cgp_core::{CanRaiseError, HasInner};

use crate::chain::traits::queries::channel_end::{
    CanQueryChannelEnd, CanQueryChannelEndWithProofs, ChannelEndQuerier,
    ChannelEndWithProofsQuerier,
};
use crate::chain::traits::types::channel::HasChannelEndType;
use crate::chain::traits::types::ibc::HasIbcChainTypes;
use crate::chain::traits::types::proof::HasCommitmentProofType;

pub struct ForwardQueryChannelEnd;

impl<Chain, InChain, Counterparty, ChannelEnd> ChannelEndQuerier<Chain, Counterparty>
    for ForwardQueryChannelEnd
where
    Chain: HasInner<Inner = InChain>
        + CanRaiseError<InChain::Error>
        + HasIbcChainTypes<Counterparty>
        + HasChannelEndType<Counterparty, ChannelEnd = ChannelEnd>,
    InChain: CanQueryChannelEnd<
            Counterparty,
            ChannelId = Chain::ChannelId,
            PortId = Chain::PortId,
            Height = Chain::Height,
        > + HasChannelEndType<Counterparty, ChannelEnd = ChannelEnd>,
{
    async fn query_channel_end(
        chain: &Chain,
        channel_id: &Chain::ChannelId,
        port_id: &Chain::PortId,
        height: &Chain::Height,
    ) -> Result<ChannelEnd, Chain::Error> {
        let channel_end = chain
            .inner()
            .query_channel_end(channel_id, port_id, height)
            .await
            .map_err(Chain::raise_error)?;

        Ok(channel_end)
    }
}

impl<Chain, InChain, Counterparty, ChannelEnd, CommitmentProof>
    ChannelEndWithProofsQuerier<Chain, Counterparty> for ForwardQueryChannelEnd
where
    Chain: HasInner<Inner = InChain>
        + CanRaiseError<InChain::Error>
        + HasIbcChainTypes<Counterparty>
        + HasCommitmentProofType<CommitmentProof = CommitmentProof>
        + HasChannelEndType<Counterparty, ChannelEnd = ChannelEnd>,
    InChain: CanQueryChannelEndWithProofs<
            Counterparty,
            ChannelId = Chain::ChannelId,
            PortId = Chain::PortId,
            Height = Chain::Height,
        > + HasChannelEndType<Counterparty, ChannelEnd = ChannelEnd>
        + HasCommitmentProofType<CommitmentProof = CommitmentProof>,
{
    async fn query_channel_end_with_proofs(
        chain: &Chain,
        channel_id: &Chain::ChannelId,
        port_id: &Chain::PortId,
        height: &Chain::Height,
    ) -> Result<(ChannelEnd, CommitmentProof), Chain::Error> {
        let result = chain
            .inner()
            .query_channel_end_with_proofs(channel_id, port_id, height)
            .await
            .map_err(Chain::raise_error)?;

        Ok(result)
    }
}
