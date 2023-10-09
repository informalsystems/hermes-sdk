use alloc::sync::Arc;

use ibc_relayer::chain::handle::ChainHandle;
use ibc_relayer_types::core::ics04_channel::packet::{Packet, PacketMsgType};
use ibc_relayer_types::Height;

use crate::contexts::chain::CosmosChain;
use crate::traits::chain_handle::HasBlockingChainHandle;
use crate::traits::message::{CosmosMessage, ToCosmosMessage};
use crate::types::error::{BaseError, Error};
use crate::types::messages::packet::timeout::CosmosTimeoutPacketMessage;
use crate::types::payloads::packet::CosmosTimeoutUnorderedPacketPayload;

pub async fn build_timeout_unordered_packet_payload<Chain: ChainHandle>(
    chain: &CosmosChain<Chain>,
    height: &Height,
    packet: &Packet,
) -> Result<CosmosTimeoutUnorderedPacketPayload, Error> {
    let height = *height;
    let packet = packet.clone();

    chain
        .with_blocking_chain_handle(move |chain_handle| {
            let proofs = chain_handle
                .build_packet_proofs(
                    PacketMsgType::TimeoutUnordered,
                    &packet.destination_port,
                    &packet.destination_channel,
                    packet.sequence,
                    height,
                )
                .map_err(BaseError::relayer)?;

            Ok(CosmosTimeoutUnorderedPacketPayload {
                update_height: proofs.height(),
                proof_unreceived: proofs.object_proof().clone(),
            })
        })
        .await
}

pub fn build_timeout_unordered_packet_message(
    packet: &Packet,
    payload: CosmosTimeoutUnorderedPacketPayload,
) -> Result<Arc<dyn CosmosMessage>, Error> {
    let message = CosmosTimeoutPacketMessage {
        next_sequence_recv: packet.sequence,
        packet: packet.clone(),
        update_height: payload.update_height,
        proof_unreceived: payload.proof_unreceived,
    };

    Ok(message.to_cosmos_message())
}
