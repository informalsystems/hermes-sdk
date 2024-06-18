use borsh::BorshSerialize;
use ibc::apps::transfer::types::msgs::transfer::MsgTransfer;
use ibc_proto::google::protobuf::Any;
use ibc_relayer_types::core::ics02_client::height::Height;
use std::io::prelude::Write;

use crate::types::message::SovereignMessage;

#[derive(Debug, BorshSerialize)]
pub enum IbcMessage {
    Core(IbcMessageWithHeight),
    Transfer(MsgTransferWithHeight),
}

#[derive(Debug)]
pub struct MsgTransferWithHeight {
    pub message: MsgTransfer,
    pub counterparty_height: Height,
}

#[derive(Debug)]
pub struct IbcMessageWithHeight {
    pub message: Any,
    pub counterparty_height: Option<Height>,
}

impl IbcMessageWithHeight {
    pub fn new(message: Any, counterparty_height: Option<Height>) -> Self {
        Self {
            message,
            counterparty_height,
        }
    }
}

impl BorshSerialize for IbcMessageWithHeight {
    fn serialize<W: Write>(&self, writer: &mut W) -> std::io::Result<()> {
        self.message.serialize(writer)
    }
}

impl BorshSerialize for MsgTransferWithHeight {
    fn serialize<W: Write>(&self, writer: &mut W) -> std::io::Result<()> {
        self.message.serialize(writer)
    }
}

impl From<IbcMessageWithHeight> for SovereignMessage {
    fn from(value: IbcMessageWithHeight) -> Self {
        SovereignMessage::Ibc(IbcMessage::Core(value))
    }
}
