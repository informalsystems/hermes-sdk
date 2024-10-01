use cgp::prelude::*;
use hermes_chain_type_components::traits::types::ibc::client_id::HasClientIdType;

use crate::traits::types::packet::nonce::HasPacketNonceType;
use crate::traits::types::packet::packet::HasPacketType;

#[derive_component(SendPacketCommitmentQuerierComponent, SendPacketCommitmentQuerier<Chain>)]
#[async_trait]
pub trait CanQuerySendPacketCommitment<Counterparty>:
    HasPacketType<Counterparty> + HasClientIdType<Counterparty> + HasErrorType
where
    Counterparty: HasClientIdType<Self> + HasPacketNonceType<Self>,
{
    async fn query_send_packet_commitment(
        &self,
        src_client_id: &Counterparty::ClientId,
        dst_client_id: &Self::ClientId,
        nonce: &Counterparty::PacketNonce,
    ) -> Result<Option<Self::Packet>, Self::Error>;
}
