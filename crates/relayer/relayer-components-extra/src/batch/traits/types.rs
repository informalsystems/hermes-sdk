use alloc::vec::Vec;

use cgp::core::Async;
use cgp::prelude::HasAsyncErrorType;
use hermes_chain_type_components::traits::types::message::HasMessageType;
use hermes_chain_type_components::traits::types::message_response::{
    HasMessageResponseType, MessageResponseOf,
};
use hermes_relayer_components::chain::types::aliases::MessageOf;
use hermes_relayer_components::multi::traits::chain_at::HasChainTypeAt;
use hermes_runtime_components::traits::channel::{HasChannelTypes, ReceiverOf, SenderOf};
use hermes_runtime_components::traits::channel_once::{HasChannelOnceTypes, SenderOnceOf};
use hermes_runtime_components::traits::runtime::HasRuntimeType;

pub trait HasMessageBatchChannelTypes<Tag>: Async {
    type BatchSubmission: Async;

    type MessageBatchSender: Async;

    type MessageBatchReceiver: Async;
}

pub type MessageBatchSenderOf<Context, Tag> =
    <Context as HasMessageBatchChannelTypes<Tag>>::MessageBatchSender;

pub type MessageBatchReceiverOf<Context, Tag> =
    <Context as HasMessageBatchChannelTypes<Tag>>::MessageBatchReceiver;

impl<Context, Tag, Chain, Runtime> HasMessageBatchChannelTypes<Tag> for Context
where
    Context:
        HasChainTypeAt<Tag, Chain = Chain> + HasRuntimeType<Runtime = Runtime> + HasAsyncErrorType,
    Chain: HasMessageType + HasMessageResponseType,
    Runtime: HasChannelTypes + HasChannelOnceTypes,
{
    type BatchSubmission = (
        Vec<Chain::Message>,
        SenderOnceOf<Runtime, Result<Vec<Chain::MessageResponse>, Context::Error>>,
    );

    type MessageBatchSender = SenderOf<Runtime, Self::BatchSubmission>;

    type MessageBatchReceiver = ReceiverOf<Runtime, Self::BatchSubmission>;
}

pub trait CanUseMessageBatchChannel<Tag>:
    HasChainTypeAt<Tag, Chain: HasMessageType + HasMessageResponseType>
    + HasRuntimeType<Runtime: HasChannelTypes + HasChannelOnceTypes>
    + HasAsyncErrorType
    + HasMessageBatchChannelTypes<
        Tag,
        BatchSubmission = (
            Vec<MessageOf<Self::Chain>>,
            SenderOnceOf<Self::Runtime, Result<Vec<MessageResponseOf<Self::Chain>>, Self::Error>>,
        ),
        MessageBatchSender = SenderOf<
            Self::Runtime,
            (
                Vec<MessageOf<Self::Chain>>,
                SenderOnceOf<
                    Self::Runtime,
                    Result<Vec<MessageResponseOf<Self::Chain>>, Self::Error>,
                >,
            ),
        >,
        MessageBatchReceiver = ReceiverOf<
            Self::Runtime,
            (
                Vec<MessageOf<Self::Chain>>,
                SenderOnceOf<
                    Self::Runtime,
                    Result<Vec<MessageResponseOf<Self::Chain>>, Self::Error>,
                >,
            ),
        >,
    >
{
}

impl<Context, Tag, Chain, Runtime> CanUseMessageBatchChannel<Tag> for Context
where
    Context:
        HasChainTypeAt<Tag, Chain = Chain> + HasRuntimeType<Runtime = Runtime> + HasAsyncErrorType,
    Chain: HasMessageType + HasMessageResponseType,
    Runtime: HasChannelTypes + HasChannelOnceTypes,
{
}
