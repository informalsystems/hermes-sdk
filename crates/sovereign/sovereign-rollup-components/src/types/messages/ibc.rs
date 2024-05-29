use borsh::BorshSerialize;
use ibc_proto::google::protobuf::Any;
use ibc_relayer_types::core::ics02_client::height::Height;

use crate::types::message::SovereignMessage;

#[derive(Debug, BorshSerialize)]
pub enum IbcMessage {
    Core(IbcMessageWithHeight),
}

#[derive(Debug)]
pub struct IbcMessageWithHeight {
    pub message: Any,
    pub counterparty_height: Option<Height>,
}

impl IbcMessageWithHeight {
    pub fn new(message: Any) -> Self {
        Self {
            message,
            counterparty_height: None,
        }
    }

    pub fn new_with_height(message: Any, height: Height) -> Self {
        Self {
            message,
            counterparty_height: Some(height),
        }
    }
}

impl BorshSerialize for IbcMessageWithHeight {
    fn serialize<W: std::io::prelude::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        self.message.serialize(writer)
    }
}

impl From<IbcMessageWithHeight> for SovereignMessage {
    fn from(value: IbcMessageWithHeight) -> Self {
        SovereignMessage::Ibc(IbcMessage::Core(value))
    }
}
