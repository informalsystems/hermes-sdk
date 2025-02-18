use cgp::prelude::*;
use hermes_chain_type_components::traits::builders::amount::CanBuildAmount;
use hermes_chain_type_components::traits::fields::amount::denom::HasAmountDenom;
use hermes_chain_type_components::traits::fields::amount::quantity::HasAmountQuantity;
use hermes_chain_type_components::traits::types::ibc::channel_id::HasChannelIdType;
use hermes_ibc_components::traits::fields::caller::HasCaller;
use hermes_ibc_components::traits::fields::message::app_id::HasIbcMessageAppIds;
use hermes_ibc_components::traits::fields::packet::header::channel_id::HasPacketChannelIds;
use hermes_ibc_components::traits::handlers::outgoing::message::{
    IbcMessageHandler, IbcMessageHandlerComponent,
};
use hermes_ibc_components::traits::types::app_id::HasAppIdType;

use crate::traits::builders::mint::CanBuildMintPayload;
use crate::traits::builders::unescrow::CanBuildUnescrowPayload;
use crate::traits::escrow_registry::escrow::CanRegisterEscrowToken;
use crate::traits::fields::message::amount::HasMessageTransferAmount;
use crate::traits::mint_registry::lookup_outgoing::CanLookupOutgoingBurnToken;
use crate::traits::token::transfer::{Burn, CanTransferToken, Escrow};

pub struct HandleOutgoingIbcTransfer;

#[cgp_provider(IbcMessageHandlerComponent)]
impl<Chain, Counterparty, App> IbcMessageHandler<Chain, Counterparty, App>
    for HandleOutgoingIbcTransfer
where
    Chain: HasPacketChannelIds<Counterparty>
        + HasIbcMessageAppIds<Counterparty>
        + HasMessageTransferAmount<Counterparty, App>
        + HasCaller
        + HasAmountDenom
        + HasAmountQuantity
        + CanLookupOutgoingBurnToken<Counterparty>
        + CanTransferToken<Burn>
        + CanTransferToken<Escrow>
        + CanRegisterEscrowToken<Counterparty>
        + CanBuildMintPayload<Counterparty, App>
        + CanBuildUnescrowPayload<Counterparty, App>,
    Counterparty: HasChannelIdType<Chain> + HasAppIdType<Chain> + CanBuildAmount,
    Chain::Quantity: Clone + Into<Counterparty::Quantity>,
{
    async fn handle_ibc_message(
        chain: &mut Chain,
        packet_header: &Chain::PacketHeader,
        message_header: &Chain::IbcMessageHeader,
        message: &Chain::IbcMessage,
    ) -> Result<(Chain::PayloadHeader, Chain::PayloadData), Chain::Error> {
        let src_channel_id = Chain::packet_src_channel_id(packet_header);
        let dst_channel_id = Chain::packet_dst_channel_id(packet_header);

        let src_app_id = Chain::ibc_message_src_app_id(message_header);
        let dst_app_id = Chain::ibc_message_dst_app_id(message_header);

        let src_amount = Chain::message_transfer_amount(message);
        let src_denom = Chain::amount_denom(src_amount);

        let sender = chain.caller();

        let m_dst_denom = chain
            .lookup_outgoing_burn_token(
                src_channel_id,
                dst_channel_id,
                src_app_id,
                dst_app_id,
                src_denom,
            )
            .await?;

        if let Some(dst_denom) = m_dst_denom {
            chain.transfer_token(Burn, &sender, src_amount).await?;

            let src_quantity = Chain::amount_quantity(src_amount);
            let dst_amount = Counterparty::build_amount(&dst_denom, &src_quantity.clone().into());

            chain.build_outgoing_unescrow_payload(message_header, message, &dst_amount)
        } else {
            chain
                .register_escrowed_token(
                    src_channel_id,
                    dst_channel_id,
                    src_app_id,
                    dst_app_id,
                    src_amount,
                )
                .await?;

            chain.transfer_token(Escrow, &sender, src_amount).await?;

            chain.build_outgoing_mint_payload(message_header, message, src_amount)
        }
    }
}
