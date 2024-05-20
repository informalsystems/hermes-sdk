use cgp_core::prelude::*;

use crate::chain::traits::types::channel::HasChannelEndType;
use crate::chain::traits::types::ibc::HasIbcChainTypes;
use crate::chain::traits::types::proof::HasCommitmentProofType;

#[derive_component(ChannelEndQuerierComponent, ChannelEndQuerier<Chain>)]
#[async_trait]
pub trait ChanQueryChannelEnd<Counterparty>:
    HasChannelEndType<Counterparty> + HasIbcChainTypes<Counterparty> + HasErrorType
{
    async fn query_channel_end(
        &self,
        channel_id: &Self::ChannelId,
        port_id: &Self::PortId,
    ) -> Result<Self::ChannelEnd, Self::Error>;
}

#[derive_component(ChannelEndWithProofsQuerierComponent, ChannelEndWithProofsQuerier<Chain>)]
#[async_trait]
pub trait CanQueryChannelEndWithProofs<Counterparty>:
    HasChannelEndType<Counterparty>
    + HasIbcChainTypes<Counterparty>
    + HasCommitmentProofType
    + HasErrorType
{
    async fn query_channel_end_with_proofs(
        &self,
        channel_id: &Self::ChannelId,
        port_id: &Self::PortId,
    ) -> Result<(Self::ChannelEnd, Self::CommitmentProof), Self::Error>;
}
