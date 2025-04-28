use alloc::sync::Arc;
use alloc::vec::Vec;

use cgp::core::Async;
use futures::channel::mpsc::{UnboundedReceiver, UnboundedSender};
use futures::channel::oneshot;
use futures::lock::Mutex;
use hermes_chain_type_components::traits::{
    HasMessageResponseType, HasMessageType, MessageResponseOf,
};
use hermes_prelude::HasAsyncErrorType;
use hermes_relayer_components::chain::types::aliases::MessageOf;
use hermes_relayer_components::multi::traits::chain_at::HasChainTypeAt;

pub trait HasMessageBatchChannelTypes<Tag>: Async {
    type BatchSubmission: Async;

    type MessageBatchSender: Async;

    type MessageBatchReceiver: Async;
}

pub type MessageBatchSenderOf<Context, Tag> =
    <Context as HasMessageBatchChannelTypes<Tag>>::MessageBatchSender;

pub type MessageBatchReceiverOf<Context, Tag> =
    <Context as HasMessageBatchChannelTypes<Tag>>::MessageBatchReceiver;

impl<Context, Tag, Chain> HasMessageBatchChannelTypes<Tag> for Context
where
    Context: HasChainTypeAt<Tag, Chain = Chain> + HasAsyncErrorType,
    Chain: HasMessageType + HasMessageResponseType,
{
    type BatchSubmission = (
        Vec<Chain::Message>,
        oneshot::Sender<Result<Vec<Chain::MessageResponse>, Context::Error>>,
    );

    type MessageBatchSender = Arc<Mutex<UnboundedSender<Self::BatchSubmission>>>;

    type MessageBatchReceiver = UnboundedReceiver<Self::BatchSubmission>;
}

pub trait CanUseMessageBatchChannel<Tag>:
    HasChainTypeAt<Tag, Chain: HasMessageType + HasMessageResponseType>
    + HasAsyncErrorType
    + HasMessageBatchChannelTypes<
        Tag,
        BatchSubmission = (
            Vec<MessageOf<Self::Chain>>,
            oneshot::Sender<Result<Vec<MessageResponseOf<Self::Chain>>, Self::Error>>,
        ),
        MessageBatchSender = Arc<
            Mutex<
                UnboundedSender<(
                    Vec<MessageOf<Self::Chain>>,
                    oneshot::Sender<Result<Vec<MessageResponseOf<Self::Chain>>, Self::Error>>,
                )>,
            >,
        >,
        MessageBatchReceiver = UnboundedReceiver<(
            Vec<MessageOf<Self::Chain>>,
            oneshot::Sender<Result<Vec<MessageResponseOf<Self::Chain>>, Self::Error>>,
        )>,
    >
{
}

impl<Context, Tag, Chain> CanUseMessageBatchChannel<Tag> for Context
where
    Context: HasChainTypeAt<Tag, Chain = Chain> + HasAsyncErrorType,
    Chain: HasMessageType + HasMessageResponseType,
{
}
