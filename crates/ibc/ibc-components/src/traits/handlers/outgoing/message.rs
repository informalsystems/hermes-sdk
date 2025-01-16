use cgp::core::component::{UseContext, UseDelegate};
use cgp::prelude::*;

use crate::traits::types::message::HasIbcMessageType;
use crate::traits::types::message_header::HasIbcMessageHeaderType;
use crate::traits::types::packet::header::HasPacketHeaderType;
use crate::traits::types::payload::data::HasPayloadDataType;
use crate::traits::types::payload::header::HasPayloadHeaderType;

#[cgp_component {
  provider: IbcMessageHandler,
  context: Chain,
}]
#[async_trait]
pub trait CanHandleIbcMessage<Counterparty, App>:
    HasAsyncErrorType
    + HasPacketHeaderType<Counterparty>
    + HasIbcMessageHeaderType<Counterparty>
    + HasIbcMessageType<Counterparty, App>
    + HasPayloadHeaderType<Counterparty>
    + HasPayloadDataType<Counterparty, App>
{
    async fn handle_ibc_message(
        &mut self,
        packet_header: &Self::PacketHeader,
        message_header: &Self::IbcMessageHeader,
        message: &Self::IbcMessage,
    ) -> Result<(Self::PayloadHeader, Self::PayloadData), Self::Error>;
}

#[async_trait]
impl<Chain, Counterparty, App> IbcMessageHandler<Chain, Counterparty, App> for UseContext
where
    Chain: CanHandleIbcMessage<Counterparty, App>,
{
    async fn handle_ibc_message(
        chain: &mut Chain,
        packet_header: &Chain::PacketHeader,
        message_header: &Chain::IbcMessageHeader,
        message: &Chain::IbcMessage,
    ) -> Result<(Chain::PayloadHeader, Chain::PayloadData), Chain::Error> {
        chain
            .handle_ibc_message(packet_header, message_header, message)
            .await
    }
}

#[async_trait]
impl<Chain, Counterparty, App, Components> IbcMessageHandler<Chain, Counterparty, App>
    for UseDelegate<Components>
where
    Chain: HasAsyncErrorType
        + HasPacketHeaderType<Counterparty>
        + HasIbcMessageHeaderType<Counterparty>
        + HasIbcMessageType<Counterparty, App>
        + HasPayloadHeaderType<Counterparty>
        + HasPayloadDataType<Counterparty, App>,
    Components: DelegateComponent<App>,
    Components::Delegate: IbcMessageHandler<Chain, Counterparty, App>,
{
    async fn handle_ibc_message(
        chain: &mut Chain,
        packet_header: &Chain::PacketHeader,
        message_header: &Chain::IbcMessageHeader,
        message: &Chain::IbcMessage,
    ) -> Result<(Chain::PayloadHeader, Chain::PayloadData), Chain::Error> {
        Components::Delegate::handle_ibc_message(chain, packet_header, message_header, message)
            .await
    }
}
