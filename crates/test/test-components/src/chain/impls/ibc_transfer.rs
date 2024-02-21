use cgp_core::prelude::*;
use cgp_core::CanRaiseError;
use hermes_relayer_components::chain::traits::queries::chain_status::CanQueryChainStatus;
use hermes_relayer_components::chain::traits::types::event::HasEventType;
use hermes_relayer_components::chain::traits::types::ibc::HasIbcChainTypes;
use hermes_relayer_components::chain::traits::types::ibc_events::send_packet::HasSendPacketEvent;
use hermes_relayer_components::chain::traits::types::message::HasMessageType;
use hermes_relayer_components::chain::traits::types::packet::HasIbcPacketTypes;
use hermes_relayer_components::transaction::components::send_single_message_with_signer::CanSendSingleMessageWithSigner;

use crate::chain::traits::messages::ibc_transfer::CanBuildIbcTokenTransferMessage;
use crate::chain::traits::transfer::ibc_transfer::TokenIbcTransferrer;
use crate::chain::traits::transfer::timeout::CanCalculateIbcTransferTimeout;
use crate::chain::traits::types::address::HasAddressType;
use crate::chain::traits::types::amount::HasAmountType;
use crate::chain::traits::types::memo::HasDefaultMemo;
use crate::chain::traits::types::tx_context::HasTxContext;
use crate::chain::traits::types::wallet::{HasWalletSigner, HasWalletType};

pub struct SendIbcTransferMessage;

#[derive(Debug)]
pub struct MissingSendPacketEventError;

#[async_trait]
impl<Chain, Counterparty, TxContext> TokenIbcTransferrer<Chain, Counterparty>
    for SendIbcTransferMessage
where
    Chain: HasTxContext<TxContext = TxContext>
        + HasWalletType
        + HasAmountType
        + HasDefaultMemo
        + HasWalletSigner
        + CanQueryChainStatus
        + CanCalculateIbcTransferTimeout
        + CanBuildIbcTokenTransferMessage<Counterparty>
        + HasIbcChainTypes<Counterparty>
        + HasIbcPacketTypes<Counterparty>
        + HasSendPacketEvent<Counterparty>
        + CanRaiseError<TxContext::Error>
        + CanRaiseError<MissingSendPacketEventError>,
    TxContext: HasMessageType<Message = Chain::Message>
        + HasEventType<Event = Chain::Event>
        + CanSendSingleMessageWithSigner,
    Counterparty: HasAddressType,
{
    async fn ibc_transfer_token(
        chain: &Chain,
        channel_id: &Chain::ChannelId,
        port_id: &Chain::PortId,
        sender_wallet: &Chain::Wallet,
        recipient_address: &Counterparty::Address,
        amount: &Chain::Amount,
    ) -> Result<Chain::OutgoingPacket, Chain::Error> {
        let tx_context = chain.tx_context();

        let chain_status = chain.query_chain_status().await?;

        let current_height = Chain::chain_status_height(&chain_status);

        let current_time = Chain::chain_status_timestamp(&chain_status);

        let timeout_height = chain.ibc_transfer_timeout_height(current_height);

        let timeout_time = chain.ibc_transfer_timeout_time(current_time);

        let memo = chain.default_memo();

        let message = chain
            .build_ibc_token_transfer_message(
                channel_id,
                port_id,
                recipient_address,
                amount,
                &memo,
                timeout_height.as_ref(),
                timeout_time.as_ref(),
            )
            .await?;

        let signer = Chain::wallet_signer(sender_wallet);

        let events = tx_context
            .send_message_with_signer(signer, message)
            .await
            .map_err(Chain::raise_error)?;

        let send_packet_event = events
            .iter()
            .find_map(Chain::try_extract_send_packet_event)
            .ok_or_else(|| Chain::raise_error(MissingSendPacketEventError))?;

        let packet = Chain::extract_packet_from_send_packet_event(&send_packet_event);

        Ok(packet)
    }
}
