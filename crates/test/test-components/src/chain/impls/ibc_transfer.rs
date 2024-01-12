use cgp_core::prelude::*;
use cgp_core::CanRaiseError;
use hermes_relayer_components::chain::traits::components::chain_status_querier::CanQueryChainStatus;
use hermes_relayer_components::chain::traits::types::ibc::HasIbcChainTypes;
use hermes_relayer_components::chain::traits::types::ibc_events::send_packet::{
    CanRaiseMissingSendPacketEventError, HasSendPacketEvent,
};
use hermes_relayer_components::chain::traits::types::packet::HasIbcPacketTypes;
use hermes_relayer_components::transaction::components::send_single_message_with_signer::CanSendSingleMessageWithSigner;

use crate::chain::traits::fields::memo::HasDefaultMemo;
use crate::chain::traits::fields::timeout::CanCalculateIbcTransferTimeout;
use crate::chain::traits::messages::ibc_transfer::CanBuildIbcTokenTransferMessage;
use crate::chain::traits::queries::ibc_transfer::TokenIbcTransferrer;
use crate::chain::traits::types::address::HasAddressType;
use crate::chain::traits::types::amount::HasAmountType;
use crate::chain::traits::types::wallet::{HasWalletSigner, HasWalletType};
use crate::driver::traits::types::chain::HasChain;
use crate::driver::traits::types::chain::HasChainType;

pub struct SendIbcTransferMessage;

#[async_trait]
impl<ChainDriver, Chain, CounterpartyDriver, Counterparty>
    TokenIbcTransferrer<ChainDriver, CounterpartyDriver> for SendIbcTransferMessage
where
    ChainDriver: HasChain<Chain = Chain>
        + CanRaiseError<Chain::Error>
        + HasWalletType
        + HasAmountType
        + HasDefaultMemo
        + HasWalletSigner
        + CanCalculateIbcTransferTimeout
        + CanBuildIbcTokenTransferMessage<CounterpartyDriver>
        + CanRaiseMissingSendPacketEventError,
    Chain: CanQueryChainStatus
        + CanSendSingleMessageWithSigner
        + HasIbcChainTypes<Counterparty>
        + HasIbcPacketTypes<Counterparty>
        + HasSendPacketEvent<Counterparty>,
    CounterpartyDriver: HasAddressType + HasChainType<Chain = Counterparty>,
{
    async fn ibc_transfer_token(
        chain_driver: &ChainDriver,
        channel_id: &Chain::ChannelId,
        port_id: &Chain::PortId,
        sender_wallet: &ChainDriver::Wallet,
        recipient_address: &CounterpartyDriver::Address,
        amount: &ChainDriver::Amount,
    ) -> Result<Chain::OutgoingPacket, ChainDriver::Error> {
        let chain = chain_driver.chain();

        let chain_status = chain
            .query_chain_status()
            .await
            .map_err(ChainDriver::raise_error)?;

        let current_height = Chain::chain_status_height(&chain_status);

        let current_time = Chain::chain_status_timestamp(&chain_status);

        let timeout_height = chain_driver.ibc_transfer_timeout_height(current_height);

        let timeout_time = chain_driver.ibc_transfer_timeout_time(current_time);

        let memo = chain_driver.default_memo();

        let sender_address = ChainDriver::wallet_address(sender_wallet);

        let message = chain_driver
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

        let signer = ChainDriver::wallet_signer(sender_wallet);

        let events = chain
            .send_message_with_signer(signer, message)
            .await
            .map_err(ChainDriver::raise_error)?;

        let send_packet_event = events
            .iter()
            .find_map(Chain::try_extract_send_packet_event)
            .ok_or_else(|| chain_driver.missing_send_packet_event_error())?;

        let packet = Chain::extract_packet_from_send_packet_event(&send_packet_event);

        Ok(packet)
    }
}
