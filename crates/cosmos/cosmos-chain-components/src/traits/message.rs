use alloc::sync::Arc;
use core::fmt::Debug;

use ibc_proto::google::protobuf::Any;
use ibc_relayer_types::signer::Signer;
use ibc_relayer_types::Height;
use prost::EncodeError;

#[derive(Debug, Clone)]
pub struct CosmosMessage {
    pub message: Arc<dyn DynCosmosMessage>,
}

impl CosmosMessage {
    pub fn new<Message: DynCosmosMessage>(message: Message) -> Self {
        Self {
            message: Arc::new(message),
        }
    }
}

pub trait DynCosmosMessage: Debug + Send + Sync + 'static {
    fn counterparty_message_height_for_update_client(&self) -> Option<Height> {
        None
    }

    fn trusted_height(&self) -> Option<Height> {
        None
    }

    fn encode_protobuf(&self, signer: &Signer) -> Result<Any, EncodeError>;
}

pub trait ToCosmosMessage {
    fn to_cosmos_message(self) -> CosmosMessage;
}

impl<Message> ToCosmosMessage for Message
where
    Message: DynCosmosMessage,
{
    fn to_cosmos_message(self) -> CosmosMessage {
        CosmosMessage::new(self)
    }
}

pub fn wrap_cosmos_message<Message: DynCosmosMessage>(message: Message) -> CosmosMessage {
    CosmosMessage::new(message)
}
