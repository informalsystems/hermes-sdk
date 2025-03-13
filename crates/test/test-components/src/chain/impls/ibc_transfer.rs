use core::marker::PhantomData;

use cgp::prelude::*;
use hermes_chain_type_components::traits::fields::message_response_events::HasMessageResponseEvents;
use hermes_relayer_components::chain::traits::extract_data::CanExtractFromEvent;
use hermes_relayer_components::chain::traits::packet::from_send_packet::CanBuildPacketFromSendPacket;
use hermes_relayer_components::chain::traits::queries::chain_status::CanQueryChainStatus;
use hermes_relayer_components::chain::traits::types::ibc::HasIbcChainTypes;
use hermes_relayer_components::chain::traits::types::ibc_events::send_packet::HasSendPacketEvent;
use hermes_relayer_components::chain::traits::types::status::HasChainStatusType;
use hermes_relayer_components::transaction::traits::send_messages_with_signer::CanSendMessagesWithSigner;

use crate::chain::traits::messages::ibc_transfer::CanBuildIbcTokenTransferMessage;
use crate::chain::traits::transfer::ibc_transfer::{
    TokenIbcTransferrer, TokenIbcTransferrerComponent,
};
use crate::chain::traits::transfer::timeout::CanCalculateIbcTransferTimeout;
use crate::chain::traits::types::address::HasAddressType;
use crate::chain::traits::types::amount::HasAmountType;
use crate::chain::traits::types::memo::HasMemoType;
use crate::chain::traits::types::wallet::{HasWalletSigner, HasWalletType};

#[derive(Debug)]
pub struct MissingSendPacketEventError;

#[cgp_new_provider(TokenIbcTransferrerComponent)]
impl<Chain, Counterparty> TokenIbcTransferrer<Chain, Counterparty> for SendIbcTransferMessage
where
    Chain: HasWalletType
        + HasAmountType
        + HasMemoType
        + HasWalletSigner
        + CanQueryChainStatus
        + CanCalculateIbcTransferTimeout
        + HasMessageResponseEvents
        + CanBuildIbcTokenTransferMessage<Counterparty>
        + HasIbcChainTypes<Counterparty>
        + HasSendPacketEvent<Counterparty>
        + CanBuildPacketFromSendPacket<Counterparty>
        + CanExtractFromEvent<Chain::SendPacketEvent>
        + CanRaiseAsyncError<MissingSendPacketEventError>
        + CanSendMessagesWithSigner,
    Counterparty: HasAddressType + HasChainStatusType,
{
    async fn ibc_transfer_token(
        chain: &Chain,
        _counterparty: PhantomData<Counterparty>,
        channel_id: &Chain::ChannelId,
        port_id: &Chain::PortId,
        sender_wallet: &Chain::Wallet,
        recipient_address: &Counterparty::Address,
        amount: &Chain::Amount,
        memo: &Chain::Memo,
        counterparty_chain_status: &Counterparty::ChainStatus,
    ) -> Result<Chain::OutgoingPacket, Chain::Error> {
        let chain_status = chain.query_chain_status().await?;

        let current_height = Chain::chain_status_height(&chain_status);

        let current_time = Chain::chain_status_time(&chain_status);

        let timeout_height = chain.ibc_transfer_timeout_height(current_height);

        let timeout_time = chain.ibc_transfer_timeout_time(current_time);

        let messages = chain
            .build_ibc_token_transfer_message(
                PhantomData,
                channel_id,
                port_id,
                recipient_address,
                amount,
                memo,
                timeout_height.as_ref(),
                timeout_time.as_ref(),
            )
            .await?;

        let signer = Chain::wallet_signer(sender_wallet);

        let responses = chain.send_messages_with_signer(signer, &messages).await?;

        let send_packet_event = responses
            .iter()
            .flat_map(Chain::message_response_events)
            .find_map(|event| chain.try_extract_from_event(PhantomData, event))
            .ok_or_else(|| Chain::raise_error(MissingSendPacketEventError))?;

        let packet = chain
            .build_packet_from_send_packet_event(&send_packet_event)
            .await?;

        Ok(packet)
    }
}
