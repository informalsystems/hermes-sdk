use core::fmt::Debug;
use core::marker::PhantomData;

use cgp::prelude::{CanRaiseError, HasErrorType};

use crate::traits::fields::message::app_id::HasIbcMessageAppIds;
use crate::traits::fields::packet::entry::app::HasPacketApplications;
use crate::traits::handlers::message::IbcMessageHandler;
use crate::traits::types::app_id::HasAppIdType;
use crate::traits::types::message::HasIbcMessageType;
use crate::traits::types::message_header::HasIbcMessageHeaderType;
use crate::traits::types::packet::data::HasPacketDataType;
use crate::traits::types::packet::entry::HasPacketEntryHeaderType;
use crate::traits::types::transaction_header::HasIbcTransactionHeaderType;

pub struct ValidateHeaderAppIds<InHandler>(pub PhantomData<InHandler>);

pub struct MismatchSrcAppId<'a, Chain, Counterparty>
where
    Chain: HasAppIdType<Counterparty>
        + HasPacketEntryHeaderType<Counterparty>
        + HasIbcMessageHeaderType<Counterparty>,
{
    pub src_message_app_id: &'a Chain::AppId,
    pub src_packet_app_id: &'a Chain::AppId,
    pub message_header: &'a Chain::IbcMessageHeader,
    pub packet_entry_header: &'a Chain::PacketEntryHeader,
}

pub struct MismatchDstAppId<'a, Chain, Counterparty>
where
    Chain: HasPacketEntryHeaderType<Counterparty> + HasIbcMessageHeaderType<Counterparty>,
    Counterparty: HasAppIdType<Chain>,
{
    pub dst_message_app_id: &'a Counterparty::AppId,
    pub dst_packet_app_id: &'a Counterparty::AppId,
    pub message_header: &'a Chain::IbcMessageHeader,
    pub packet_entry_header: &'a Chain::PacketEntryHeader,
}

impl<Chain, Counterparty, App, InHandler> IbcMessageHandler<Chain, Counterparty, App>
    for ValidateHeaderAppIds<InHandler>
where
    Chain: HasErrorType
        + HasIbcTransactionHeaderType<Counterparty>
        + HasIbcMessageHeaderType<Counterparty>
        + HasIbcMessageType<Counterparty, App>
        + HasPacketDataType<Counterparty, App>
        + HasPacketEntryHeaderType<Counterparty>
        + HasIbcMessageAppIds<Counterparty>
        + HasPacketApplications<Counterparty>
        + for<'a> CanRaiseError<MismatchSrcAppId<'a, Chain, Counterparty>>
        + for<'a> CanRaiseError<MismatchDstAppId<'a, Chain, Counterparty>>,
    Counterparty: HasAppIdType<Chain>,
    InHandler: IbcMessageHandler<Chain, Counterparty, App>,
    Chain::AppId: Eq,
    Counterparty::AppId: Eq,
{
    async fn handle_ibc_message(
        chain: &Chain,
        transaction_header: &Chain::IbcTransactionHeader,
        message_header: &Chain::IbcMessageHeader,
        message: &Chain::IbcMessage,
    ) -> Result<(Chain::PacketEntryHeader, Chain::PacketData), Chain::Error> {
        let (packet_entry_header, packet_data) =
            InHandler::handle_ibc_message(chain, transaction_header, message_header, message)
                .await?;

        let src_message_app_id = Chain::ibc_message_src_app_id(message_header);
        let dst_message_app_id = Chain::ibc_message_dst_app_id(message_header);

        let src_packet_app_id = Chain::packet_src_app_id(&packet_entry_header);
        let dst_packet_app_id = Chain::packet_dst_app_id(&packet_entry_header);

        if src_message_app_id != src_packet_app_id {
            return Err(Chain::raise_error(MismatchSrcAppId {
                src_message_app_id,
                src_packet_app_id,
                message_header,
                packet_entry_header: &packet_entry_header,
            }));
        }

        if dst_message_app_id != dst_packet_app_id {
            return Err(Chain::raise_error(MismatchDstAppId {
                dst_message_app_id,
                dst_packet_app_id,
                message_header,
                packet_entry_header: &packet_entry_header,
            }));
        }

        Ok((packet_entry_header, packet_data))
    }
}

impl<'a, Chain, Counterparty> Debug for MismatchSrcAppId<'a, Chain, Counterparty>
where
    Chain: HasAppIdType<Counterparty>
        + HasPacketEntryHeaderType<Counterparty>
        + HasIbcMessageHeaderType<Counterparty>,
    Chain::AppId: Debug,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "packet entry header produced by application has a different source app ID ({:?}) specified in message header ({:?})",
            self.src_message_app_id,
            self.src_packet_app_id,
        )
    }
}

impl<'a, Chain, Counterparty> Debug for MismatchDstAppId<'a, Chain, Counterparty>
where
    Chain: HasPacketEntryHeaderType<Counterparty> + HasIbcMessageHeaderType<Counterparty>,
    Counterparty: HasAppIdType<Chain>,
    Counterparty::AppId: Debug,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "packet entry header produced by application has a different destination app ID ({:?}) specified in message header ({:?})",
            self.dst_message_app_id,
            self.dst_packet_app_id,
        )
    }
}
