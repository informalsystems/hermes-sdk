use cgp_core::CanRaiseError;
use hermes_relayer_components::chain::traits::message_builders::receive_packet::ReceivePacketMessageBuilder;
use hermes_relayer_components::chain::traits::types::height::HasHeightFields;
use hermes_relayer_components::chain::traits::types::message::HasMessageType;
use hermes_relayer_components::chain::traits::types::packet::HasIbcPacketTypes;
use hermes_relayer_components::chain::traits::types::packets::receive::HasReceivePacketPayloadType;
use hermes_relayer_components::chain::traits::types::proof::HasCommitmentProofType;
use hermes_relayer_components::chain::types::payloads::packet::ReceivePacketPayload;
use ibc_relayer_types::core::ics02_client::error::Error as Ics02Error;
use ibc_relayer_types::core::ics04_channel::packet::Packet;
use ibc_relayer_types::Height;

use crate::traits::message::{CosmosMessage, ToCosmosMessage};
use crate::types::messages::packet::receive::CosmosReceivePacketMessage;

pub struct BuildCosmosPacketMessages;

impl<Chain, Counterparty> ReceivePacketMessageBuilder<Chain, Counterparty>
    for BuildCosmosPacketMessages
where
    Chain: HasMessageType<Message = CosmosMessage>
        + HasIbcPacketTypes<Counterparty, IncomingPacket = Packet>
        + CanRaiseError<Ics02Error>,
    Counterparty: HasReceivePacketPayloadType<
            Chain,
            ReceivePacketPayload = ReceivePacketPayload<Counterparty>,
        > + HasHeightFields
        + HasCommitmentProofType<CommitmentProof = Vec<u8>>,
{
    async fn build_receive_packet_message(
        _chain: &Chain,
        packet: &Packet,
        payload: ReceivePacketPayload<Counterparty>,
    ) -> Result<Chain::Message, Chain::Error> {
        let update_height = Height::new(
            Counterparty::revision_number(&payload.update_height),
            Counterparty::revision_height(&payload.update_height),
        )
        .map_err(Chain::raise_error)?;

        let message = CosmosReceivePacketMessage {
            packet: packet.clone(),
            update_height,
            proof_commitment: payload.proof_commitment,
        };

        Ok(message.to_cosmos_message())
    }
}
