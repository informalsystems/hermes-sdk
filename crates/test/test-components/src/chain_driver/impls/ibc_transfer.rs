use cgp_core::prelude::*;
use cgp_core::CanRaiseError;
use hermes_relayer_components::chain::traits::queries::chain_status::CanQueryChainStatus;
use hermes_relayer_components::chain::traits::types::event::HasEventType;
use hermes_relayer_components::chain::traits::types::ibc::HasIbcChainTypes;
use hermes_relayer_components::chain::traits::types::ibc_events::send_packet::HasSendPacketEvent;
use hermes_relayer_components::chain::traits::types::message::HasMessageType;
use hermes_relayer_components::chain::traits::types::packet::HasIbcPacketTypes;
use hermes_relayer_components::transaction::components::send_single_message_with_signer::CanSendSingleMessageWithSigner;

use crate::chain_driver::traits::fields::memo::HasDefaultMemo;
use crate::chain_driver::traits::fields::timeout::CanCalculateIbcTransferTimeout;
use crate::chain_driver::traits::messages::ibc_transfer::CanBuildIbcTokenTransferMessage;
use crate::chain_driver::traits::queries::ibc_transfer::TokenIbcTransferrer;
use crate::chain_driver::traits::types::address::HasAddressType;
use crate::chain_driver::traits::types::amount::HasAmountType;
use crate::chain_driver::traits::types::chain::{HasChain, HasChainType};
use crate::chain_driver::traits::types::tx_context::HasTxContext;
use crate::chain_driver::traits::types::wallet::{HasWalletSigner, HasWalletType};

pub struct SendIbcTransferMessage;

#[derive(Debug)]
pub struct MissingSendPacketEventError;

#[async_trait]
impl<ChainDriver, Chain, CounterpartyDriver, Counterparty, TxContext>
    TokenIbcTransferrer<ChainDriver, CounterpartyDriver> for SendIbcTransferMessage
where
    ChainDriver: HasChain<Chain = Chain>
        + HasTxContext<TxContext = TxContext>
        + CanRaiseError<Chain::Error>
        + CanRaiseError<TxContext::Error>
        + HasWalletType
        + HasAmountType
        + HasDefaultMemo
        + HasWalletSigner
        + CanCalculateIbcTransferTimeout
        + CanBuildIbcTokenTransferMessage<CounterpartyDriver>
        + CanRaiseError<MissingSendPacketEventError>,
    Chain: CanQueryChainStatus
        + HasIbcChainTypes<Counterparty>
        + HasIbcPacketTypes<Counterparty>
        + HasSendPacketEvent<Counterparty>,
    TxContext: HasMessageType<Message = Chain::Message>
        + HasEventType<Event = Chain::Event>
        + CanSendSingleMessageWithSigner,
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
        let tx_context = chain_driver.tx_context();

        let chain_status = chain
            .query_chain_status()
            .await
            .map_err(ChainDriver::raise_error)?;

        let current_height = Chain::chain_status_height(&chain_status);

        let current_time = Chain::chain_status_timestamp(&chain_status);

        let timeout_height = chain_driver.ibc_transfer_timeout_height(current_height);

        let timeout_time = chain_driver.ibc_transfer_timeout_time(current_time);

        let memo = chain_driver.default_memo();

        let message = chain_driver
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

        let signer = ChainDriver::wallet_signer(sender_wallet);

        let events = tx_context
            .send_message_with_signer(signer, message)
            .await
            .map_err(ChainDriver::raise_error)?;

        let send_packet_event = events
            .iter()
            .find_map(Chain::try_extract_send_packet_event)
            .ok_or_else(|| ChainDriver::raise_error(MissingSendPacketEventError))?;

        let packet = Chain::extract_packet_from_send_packet_event(&send_packet_event);

        Ok(packet)
    }
}
