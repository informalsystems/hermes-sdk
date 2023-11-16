use alloc::boxed::Box;
use cgp_core::prelude::*;
use ibc_relayer_components::chain::traits::components::chain_status_querier::CanQueryChainStatus;
use ibc_relayer_components::chain::traits::types::ibc::HasIbcChainTypes;
use ibc_relayer_components::chain::traits::types::ibc_events::send_packet::{
    CanRaiseMissingSendPacketEventError, HasSendPacketEvent,
};
use ibc_relayer_components::chain::traits::types::packet::HasIbcPacketTypes;
use ibc_relayer_components::transaction::components::send_single_message_with_signer::CanSendSingleMessageWithSigner;

use crate::traits::chain::fields::memo::HasDefaultMemo;
use crate::traits::chain::fields::timeout::CanCalculateIbcTransferTimeout;
use crate::traits::chain::messages::ibc_transfer::CanBuildIbcTokenTransferMessage;
use crate::traits::chain::queries::ibc_transfer::TokenIbcTransferrer;
use crate::traits::chain::types::address::HasAddressType;
use crate::traits::chain::types::amount::HasAmountType;
use crate::traits::chain::types::wallet::{HasWalletSigner, HasWalletType};

pub struct SendIbcTransferMessage;

#[async_trait]
impl<Chain, Counterparty> TokenIbcTransferrer<Chain, Counterparty> for SendIbcTransferMessage
where
    Chain: HasErrorType
        + HasWalletType
        + HasAmountType
        + HasDefaultMemo
        + HasWalletSigner
        + CanQueryChainStatus
        + CanCalculateIbcTransferTimeout
        + CanSendSingleMessageWithSigner
        + CanRaiseMissingSendPacketEventError
        + HasIbcChainTypes<Counterparty>
        + HasIbcPacketTypes<Counterparty>
        + HasSendPacketEvent<Counterparty>
        + CanBuildIbcTokenTransferMessage<Counterparty>,
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
        let chain_status = chain.query_chain_status().await?;

        let current_height = Chain::chain_status_height(&chain_status);

        let current_time = Chain::chain_status_timestamp(&chain_status);

        let timeout_height = chain.ibc_transfer_timeout_height(current_height);

        let timeout_time = chain.ibc_transfer_timeout_time(current_time);

        let memo = chain.default_memo();

        let sender_address = Chain::wallet_address(sender_wallet);

        let message = chain
            .build_ibc_token_transfer_message(
                channel_id,
                port_id,
                sender_address,
                recipient_address,
                amount,
                &memo,
                timeout_height.as_ref(),
                timeout_time.as_ref(),
            )
            .await?;

        let signer = Chain::wallet_signer(sender_wallet);

        let events = chain.send_message_with_signer(signer, message).await?;

        let send_packet_event = events
            .iter()
            .find_map(Chain::try_extract_send_packet_event)
            .ok_or_else(|| chain.missing_send_packet_event_error())?;

        let packet = Chain::extract_packet_from_send_packet_event(&send_packet_event);

        Ok(packet)
    }
}
