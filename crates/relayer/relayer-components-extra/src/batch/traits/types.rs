use alloc::vec::Vec;

use cgp::prelude::HasErrorType;
use hermes_chain_type_components::traits::types::message::HasMessageType;
use hermes_chain_type_components::traits::types::message_response::{
    HasMessageResponseType, MessageResponseOf,
};
use hermes_relayer_components::chain::types::aliases::MessageOf;
use hermes_relayer_components::multi::traits::chain_at::HasChainTypeAt;
use hermes_runtime_components::traits::channel::{HasChannelTypes, ReceiverOf, SenderOf};
use hermes_runtime_components::traits::channel_once::{HasChannelOnceTypes, SenderOnceOf};
use hermes_runtime_components::traits::runtime::HasRuntime;

pub trait HasMessageBatchChannelTypes<Tag> {
    type MessageBatchSender;

    type MessageBatchReceiver;
}

impl<Context, Tag, Chain, Runtime> HasMessageBatchChannelTypes<Tag> for Context
where
    Context: HasChainTypeAt<Tag, Chain = Chain> + HasRuntime<Runtime = Runtime> + HasErrorType,
    Chain: HasMessageType + HasMessageResponseType,
    Runtime: HasChannelTypes + HasChannelOnceTypes,
{
    type MessageBatchSender = SenderOf<
        Runtime,
        (
            Vec<Chain::Message>,
            SenderOnceOf<Runtime, Result<Chain::MessageResponse, Context::Error>>,
        ),
    >;

    type MessageBatchReceiver = ReceiverOf<
        Runtime,
        (
            Vec<Chain::Message>,
            SenderOnceOf<Runtime, Result<Chain::MessageResponse, Context::Error>>,
        ),
    >;
}

pub trait CanUseMessageBatchChannelTypes<Tag>:
    HasChainTypeAt<Tag, Chain: HasMessageType + HasMessageResponseType>
    + HasRuntime<Runtime: HasChannelTypes + HasChannelOnceTypes>
    + HasErrorType
    + HasMessageBatchChannelTypes<
        Tag,
        MessageBatchSender = SenderOf<
            Self::Runtime,
            (
                Vec<MessageOf<Self::Chain>>,
                SenderOnceOf<Self::Runtime, Result<MessageResponseOf<Self::Chain>, Self::Error>>,
            ),
        >,
        MessageBatchReceiver = ReceiverOf<
            Self::Runtime,
            (
                Vec<MessageOf<Self::Chain>>,
                SenderOnceOf<Self::Runtime, Result<MessageResponseOf<Self::Chain>, Self::Error>>,
            ),
        >,
    >
{
}

impl<Context, Tag, Chain, Runtime> CanUseMessageBatchChannelTypes<Tag> for Context
where
    Context: HasChainTypeAt<Tag, Chain = Chain> + HasRuntime<Runtime = Runtime> + HasErrorType,
    Chain: HasMessageType + HasMessageResponseType,
    Runtime: HasChannelTypes + HasChannelOnceTypes,
{
}
