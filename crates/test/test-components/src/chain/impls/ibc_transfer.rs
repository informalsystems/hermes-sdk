use core::marker::PhantomData;

use hermes_chain_type_components::traits::{
    HasAddressType, HasAmountType, HasHeightType, HasMessageResponseEvents, HasTimeoutType,
};
use hermes_prelude::*;
use hermes_relayer_components::chain::traits::{
    CanBuildPacketFromSendPacket, CanExtractFromEvent, HasChainStatusType, HasChannelIdType,
    HasPortIdType, HasSendPacketEvent,
};
use hermes_relayer_components::transaction::traits::CanSendMessagesWithSigner;

use crate::chain::traits::{
    CanBuildIbcTokenTransferMessages, CanCalculateIbcTransferTimeout, HasMemoType, HasWalletSigner,
    HasWalletType, TokenIbcTransferrer, TokenIbcTransferrerComponent,
};

#[derive(Debug)]
pub struct MissingSendPacketEventError;

#[cgp_new_provider(TokenIbcTransferrerComponent)]
impl<Chain, Counterparty> TokenIbcTransferrer<Chain, Counterparty> for SendIbcTransferMessage
where
    Chain: HasWalletType
        + HasAmountType
        + HasMemoType
        + HasWalletSigner
        + HasMessageResponseEvents
        + CanCalculateIbcTransferTimeout<Counterparty>
        + CanBuildIbcTokenTransferMessages<Counterparty>
        + HasPortIdType<Counterparty>
        + HasChannelIdType<Counterparty>
        + HasSendPacketEvent<Counterparty>
        + CanBuildPacketFromSendPacket<Counterparty>
        + CanExtractFromEvent<Chain::SendPacketEvent>
        + CanRaiseAsyncError<MissingSendPacketEventError>
        + CanSendMessagesWithSigner,
    Counterparty: HasAddressType + HasChainStatusType + HasTimeoutType + HasHeightType,
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
        counterparty_status: &Counterparty::ChainStatus,
    ) -> Result<Chain::OutgoingPacket, Chain::Error> {
        let timeout_height = chain
            .ibc_transfer_timeout_height(Counterparty::chain_status_height(counterparty_status));

        let timeout_time =
            chain.ibc_transfer_timeout_time(Counterparty::chain_status_time(counterparty_status));

        let messages = chain
            .build_ibc_token_transfer_messages(
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
