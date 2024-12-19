use core::fmt::Debug;

use cgp::prelude::*;
use hermes_chain_type_components::traits::types::height::HasHeightType;
use hermes_chain_type_components::traits::types::ibc::channel_id::HasChannelIdType;
use hermes_chain_type_components::traits::types::ibc::port_id::HasPortIdType;

use crate::traits::types::channel::HasChannelEndType;
use crate::traits::types::proof::HasCommitmentProofType;

#[cgp_component {
  provider: ChannelEndQuerier,
  context: Chain,
}]
#[async_trait]
pub trait CanQueryChannelEnd<Counterparty>:
    HasHeightType
    + HasChannelIdType<Counterparty>
    + HasPortIdType<Counterparty>
    + HasChannelEndType<Counterparty>
    + HasErrorType
{
    async fn query_channel_end(
        &self,
        channel_id: &Self::ChannelId,
        port_id: &Self::PortId,
        height: &Self::Height,
    ) -> Result<Self::ChannelEnd, Self::Error>;
}

#[cgp_component {
  provider: ChannelEndWithProofsQuerier,
  context: Chain,
}]
#[async_trait]
pub trait CanQueryChannelEndWithProofs<Counterparty>:
    HasHeightType
    + HasChannelIdType<Counterparty>
    + HasPortIdType<Counterparty>
    + HasChannelEndType<Counterparty>
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
    Chain: HasHeightType + HasChannelIdType<Counterparty> + HasPortIdType<Counterparty>,
{
    pub chain: &'a Chain,
    pub channel_id: &'a Chain::ChannelId,
    pub port_id: &'a Chain::PortId,
    pub height: &'a Chain::Height,
}

impl<Chain, Counterparty> Debug for ChannelNotFoundError<'_, Chain, Counterparty>
where
    Chain: HasHeightType + HasChannelIdType<Counterparty> + HasPortIdType<Counterparty>,
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
