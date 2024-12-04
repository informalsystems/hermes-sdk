use core::fmt::Debug;

use cgp::prelude::*;

use crate::traits::types::channel::HasChannelEndType;
use crate::traits::types::ibc::HasIbcChainTypes;
use crate::traits::types::proof::HasCommitmentProofType;

#[derive_component(ChannelEndQuerierComponent, ChannelEndQuerier<Chain>)]
#[async_trait]
pub trait CanQueryChannelEnd<Counterparty>:
    HasChannelEndType<Counterparty> + HasIbcChainTypes<Counterparty> + HasErrorType
{
    async fn query_channel_end(
        &self,
        channel_id: &Self::ChannelId,
        port_id: &Self::PortId,
        height: &Self::Height,
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
        height: &Self::Height,
    ) -> Result<(Self::ChannelEnd, Self::CommitmentProof), Self::Error>;
}

pub struct ChannelNotFoundError<'a, Chain, Counterparty>
where
    Chain: HasIbcChainTypes<Counterparty>,
{
    pub chain: &'a Chain,
    pub channel_id: &'a Chain::ChannelId,
    pub port_id: &'a Chain::PortId,
    pub height: &'a Chain::Height,
}

impl<Chain, Counterparty> Debug for ChannelNotFoundError<'_, Chain, Counterparty>
where
    Chain: HasIbcChainTypes<Counterparty>,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        writeln!(
            f,
            "channel not found with channel id {:?}, port id {}, height {}",
            self.channel_id, self.port_id, self.height,
        )?;

        Ok(())
    }
}
