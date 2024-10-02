use core::fmt::Debug;

use cgp::prelude::{CanRaiseError, HasErrorType};
use hermes_chain_type_components::traits::builders::amount::CanBuildAmount;
use hermes_chain_type_components::traits::fields::amount::denom::HasAmountDenom;
use hermes_chain_type_components::traits::fields::amount::quantity::HasAmountQuantity;
use hermes_chain_type_components::traits::types::address::HasAddressType;
use hermes_chain_type_components::traits::types::amount::HasAmountType;
use hermes_chain_type_components::traits::types::ibc::channel_id::HasChannelIdType;
use hermes_ibc_components::traits::fields::packet::header::channel::HasPacketChannels;
use hermes_ibc_components::traits::fields::payload::app::HasPayloadAppIds;
use hermes_ibc_components::traits::handlers::incoming::payload::IncomingPayloadHandler;
use hermes_ibc_components::traits::types::app_id::HasAppIdType;
use hermes_ibc_components::traits::types::packet::header::HasPacketHeaderType;
use hermes_ibc_components::traits::types::payload::ack::HasPayloadAckType;
use hermes_ibc_components::traits::types::payload::data::HasPayloadDataType;
use hermes_ibc_components::traits::types::payload::header::HasPayloadHeaderType;

use crate::traits::escrow_registry::lookup::CanLookupEscrowedToken;
use crate::traits::escrow_registry::update::{CanUpdateEscrowedToken, Decrease};
use crate::traits::fields::payload_data::amount::{HasIbcTransferAmount, IbcTransferAmount};
use crate::traits::fields::payload_data::receiver::HasIbcTransferReceiver;
use crate::traits::mint_registry::lookup::CanLookupMintedToken;
use crate::traits::mint_registry::register::CanRegisterMintedToken;
use crate::traits::token::create::CanCreateToken;
use crate::traits::token::transfer::{CanTransferToken, Mint, Unescrow};

pub struct HandleIncomingIbcTransfer;

pub struct UnescrowAmountExceeded<'a, Chain, Counterparty>
where
    Chain: HasAmountType + HasChannelIdType<Counterparty> + HasAppIdType<Counterparty>,
    Counterparty: HasChannelIdType<Chain> + HasAppIdType<Chain>,
{
    pub amount: &'a Chain::Amount,
    pub src_channel_id: &'a Counterparty::ChannelId,
    pub dst_channel_id: &'a Chain::ChannelId,
    pub src_app_id: &'a Counterparty::AppId,
    pub dst_app_id: &'a Chain::AppId,
}

impl<Chain, Counterparty, App> IncomingPayloadHandler<Chain, Counterparty, App>
    for HandleIncomingIbcTransfer
where
    Chain: HasErrorType
        + HasAmountType
        + HasAmountDenom
        + HasAmountQuantity
        + HasAddressType
        + CanBuildAmount
        + CanCreateToken
        + HasChannelIdType<Counterparty>
        + HasAppIdType<Counterparty>
        + HasPayloadAckType<Counterparty, App, PayloadAck = ()>
        + CanTransferToken<Mint>
        + CanTransferToken<Unescrow>
        + CanLookupMintedToken<Counterparty>
        + CanRegisterMintedToken<Counterparty>
        + CanLookupEscrowedToken<Counterparty>
        + CanUpdateEscrowedToken<Counterparty, Decrease>
        + for<'a> CanRaiseError<UnescrowAmountExceeded<'a, Chain, Counterparty>>,
    Counterparty: HasAmountDenom
        + HasAmountQuantity
        + HasChannelIdType<Chain>
        + HasAppIdType<Chain>
        + HasPacketHeaderType<Chain>
        + HasPayloadHeaderType<Chain>
        + HasPacketChannels<Chain>
        + HasPayloadAppIds<Chain>
        + HasPayloadDataType<Chain, App>
        + HasIbcTransferAmount<Chain, App>
        + HasIbcTransferReceiver<Chain, App>,
    Chain::Quantity: Default + Ord + for<'a> From<&'a Counterparty::Quantity>,
{
    async fn handle_incoming_payload(
        chain: &Chain,
        packet_header: &Counterparty::PacketHeader,
        payload_header: &Counterparty::PayloadHeader,
        payload_data: &Counterparty::PayloadData,
    ) -> Result<(), Chain::Error> {
        let src_channel_id = Counterparty::packet_src_channel_id(packet_header);
        let dst_channel_id = Counterparty::packet_dst_channel_id(packet_header);

        let src_app_id = Counterparty::payload_src_app_id(payload_header);
        let dst_app_id = Counterparty::payload_dst_app_id(payload_header);

        let receiver = Counterparty::ibc_transfer_receiver(payload_data);
        let amount = Counterparty::ibc_transfer_amount(payload_data);

        match amount {
            IbcTransferAmount::Mint(amount) => {
                let src_denom = Counterparty::amount_denom(amount);
                let quantity = Counterparty::amount_quantity(amount);

                let m_dst_denom = chain
                    .lookup_minted_token(
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
                        let dst_denom = chain.create_token().await?;

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

                let amount = Chain::build_amount(&dst_denom, &quantity.into());
                chain.transfer_token(Mint, receiver, &amount).await?;

                Ok(())
            }
            IbcTransferAmount::Unescrow(amount) => {
                let denom = Chain::amount_denom(amount);
                let quantity = Chain::amount_quantity(amount);

                let total_escrowed_quantity = chain
                    .lookup_escrowed_token(
                        src_channel_id,
                        dst_channel_id,
                        src_app_id,
                        dst_app_id,
                        denom,
                    )
                    .await?
                    .unwrap_or_default();

                if quantity > &total_escrowed_quantity {
                    return Err(Chain::raise_error(UnescrowAmountExceeded {
                        amount,
                        src_channel_id,
                        dst_channel_id,
                        src_app_id,
                        dst_app_id,
                    }));
                }

                chain.transfer_token(Unescrow, receiver, amount).await?;

                chain
                    .update_escrowed_token(
                        Decrease,
                        src_channel_id,
                        dst_channel_id,
                        src_app_id,
                        dst_app_id,
                        amount,
                    )
                    .await?;

                Ok(())
            }
        }
    }
}

impl<'a, Chain, Counterparty> Debug for UnescrowAmountExceeded<'a, Chain, Counterparty>
where
    Chain: HasAmountType + HasChannelIdType<Counterparty> + HasAppIdType<Counterparty>,
    Counterparty: HasChannelIdType<Chain> + HasAppIdType<Chain>,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "requested unescrow amount {} exceeds the total escrowed amount for the given channel",
            self.amount
        )
    }
}
