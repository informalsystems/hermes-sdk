use cgp::prelude::*;
use hermes_chain_type_components::traits::fields::amount::denom::HasAmountDenom;
use hermes_chain_type_components::traits::fields::amount::quantity::HasAmountQuantity;
use hermes_ibc_components::traits::fields::packet::header::channel_id::HasPacketChannelIds;
use hermes_ibc_components::traits::fields::payload::app_id::HasPayloadAppIds;
use hermes_ibc_components::traits::handlers::incoming::payload::{
    IncomingPayloadHandler, IncomingPayloadHandlerComponent,
};
use hermes_ibc_components::traits::types::payload::data::HasPayloadDataType;

use crate::traits::escrow_registry::unescrow::CanRegisterUnescrowToken;
use crate::traits::fields::payload_data::receiver::HasIbcTransferReceiver;
use crate::traits::fields::payload_data::unescrow_amount::HasPayloadUnescrowAmount;
use crate::traits::token::transfer::{CanTransferToken, Unescrow};

pub struct HandleIncomingUnescrowTransfer;

#[cgp_provider(IncomingPayloadHandlerComponent)]
impl<Chain, Counterparty, App> IncomingPayloadHandler<Chain, Counterparty, App>
    for HandleIncomingUnescrowTransfer
where
    Chain: HasAmountDenom
        + HasAmountQuantity
        + CanTransferToken<Unescrow>
        + CanRegisterUnescrowToken<Counterparty>,
    Counterparty: HasAmountDenom
        + HasAmountQuantity
        + HasPacketChannelIds<Chain>
        + HasPayloadAppIds<Chain>
        + HasPayloadDataType<Chain, App>
        + HasIbcTransferReceiver<Chain, App>
        + HasPayloadUnescrowAmount<Chain, App>,
    Chain::Quantity: Default + Ord,
{
    async fn handle_incoming_payload(
        chain: &mut Chain,
        packet_header: &Counterparty::PacketHeader,
        payload_header: &Counterparty::PayloadHeader,
        payload_data: &Counterparty::PayloadData,
    ) -> Result<(), Chain::Error> {
        let src_channel_id = Counterparty::packet_src_channel_id(packet_header);
        let dst_channel_id = Counterparty::packet_dst_channel_id(packet_header);

        let src_app_id = Counterparty::payload_src_app_id(payload_header);
        let dst_app_id = Counterparty::payload_dst_app_id(payload_header);

        let receiver = Counterparty::ibc_transfer_receiver(payload_data);
        let amount = Counterparty::payload_unescrow_amount(payload_data);

        chain
            .register_unescrow_token(
                src_channel_id,
                dst_channel_id,
                src_app_id,
                dst_app_id,
                amount,
            )
            .await?;

        chain.transfer_token(Unescrow, receiver, amount).await?;

        Ok(())
    }
}
