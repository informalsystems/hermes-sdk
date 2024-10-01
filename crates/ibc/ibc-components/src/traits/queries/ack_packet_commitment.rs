use cgp::prelude::*;
use hermes_chain_type_components::traits::types::ibc::client_id::HasClientIdType;

use crate::traits::types::packet::ack::HasPacketAckType;
use crate::traits::types::packet::nonce::HasPacketNonceType;

#[derive_component(AckPacketCommitmentQuerierComponent, AckPacketCommitmentQuerier<Chain>)]
#[async_trait]
pub trait CanQueryAckPacketCommitment<Counterparty>:
    HasPacketAckType<Counterparty> + HasClientIdType<Counterparty> + HasErrorType
where
    Counterparty: HasClientIdType<Self> + HasPacketNonceType<Self>,
{
    async fn query_ack_packet_commitment(
        &self,
        src_client_id: &Counterparty::ClientId,
        dst_client_id: &Self::ClientId,
        nonce: &Counterparty::PacketNonce,
    ) -> Result<Option<Self::PacketAck>, Self::Error>;
}
