use cgp::prelude::*;
use hermes_chain_type_components::traits::{CanBuildAmount, HasAmountDenom, HasAmountQuantity};
use hermes_ibc_components::traits::fields::packet::header::channel_id::HasPacketChannelIds;
use hermes_ibc_components::traits::fields::payload::app_id::HasPayloadAppIds;
use hermes_ibc_components::traits::handlers::incoming::payload::{
    IncomingPayloadHandler, IncomingPayloadHandlerComponent,
};

use crate::traits::fields::payload_data::mint_amount::HasPayloadMintAmount;
use crate::traits::fields::payload_data::receiver::HasIbcTransferReceiver;
use crate::traits::mint_registry::lookup_incoming::CanLookupIncomingMintedToken;
use crate::traits::mint_registry::register::CanRegisterMintedToken;
use crate::traits::token::create::CanCreateToken;
use crate::traits::token::transfer::{CanTransferToken, Mint};

pub struct HandleIncomingMintTransfer;

#[cgp_provider(IncomingPayloadHandlerComponent)]
impl<Chain, Counterparty, App> IncomingPayloadHandler<Chain, Counterparty, App>
    for HandleIncomingMintTransfer
where
    Chain: HasAmountDenom
        + HasAmountQuantity
        + CanBuildAmount
        + CanCreateToken<Counterparty>
        + CanTransferToken<Mint>
        + CanLookupIncomingMintedToken<Counterparty>
        + CanRegisterMintedToken<Counterparty>,
    Counterparty: HasAmountDenom
        + HasAmountQuantity
        + HasPacketChannelIds<Chain>
        + HasPayloadAppIds<Chain>
        + HasPayloadMintAmount<Chain, App>
        + HasIbcTransferReceiver<Chain, App>,
    Chain::Quantity: From<Counterparty::Quantity>,
    Counterparty::Quantity: Clone,
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

        let amount = Counterparty::payload_mint_amount(payload_data);

        let src_denom = Counterparty::amount_denom(amount);
        let quantity = Counterparty::amount_quantity(amount);

        let m_dst_denom = chain
            .lookup_incoming_minted_token(
                src_channel_id,
                dst_channel_id,
                src_app_id,
                dst_app_id,
                src_denom,
            )
            .await?;

        let dst_denom = match m_dst_denom {
            Some(dst_denom) => dst_denom,
            None => {
                let dst_denom = chain
                    .create_token(
                        src_channel_id,
                        dst_channel_id,
                        src_app_id,
                        dst_app_id,
                        src_denom,
                    )
                    .await?;

                chain
                    .register_minted_token(
                        src_channel_id,
                        dst_channel_id,
                        src_app_id,
                        dst_app_id,
                        src_denom,
                        &dst_denom,
                    )
                    .await?;

                dst_denom
            }
        };

        let amount = Chain::build_amount(&dst_denom, &quantity.clone().into());
        chain.transfer_token(Mint, receiver, &amount).await?;

        Ok(())
    }
}
