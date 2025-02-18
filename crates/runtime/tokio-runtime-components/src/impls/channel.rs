use cgp::prelude::*;
use hermes_async_runtime_components::channel::types::ErrChannelClosed;
use hermes_async_runtime_components::stream::traits::boxed::HasBoxedStreamType;
use hermes_runtime_components::traits::channel::{
    ChannelCreator, ChannelCreatorComponent, ChannelTypeComponent, ChannelUser,
    ChannelUserComponent, ProvideChannelType, ReceiverStreamer, ReceiverStreamerComponent,
    SenderCloner, SenderClonerComponent,
};
use tokio::sync::mpsc;
use tokio_stream::wrappers::UnboundedReceiverStream;

use crate::traits::channel::{HasUnboundedChannelType, UnboundedChannelTypeProvider};

pub struct ProvideUnboundedChannelType;

#[cgp_provider(ChannelTypeComponent)]
impl<Runtime> ProvideChannelType<Runtime> for ProvideUnboundedChannelType
where
    Runtime: Async,
{
    type Sender<T>
        = mpsc::UnboundedSender<T>
    where
        T: Async;

    type Receiver<T>
        = mpsc::UnboundedReceiver<T>
    where
        T: Async;
}

impl<Runtime> UnboundedChannelTypeProvider<Runtime> for ProvideUnboundedChannelType
where
    Runtime: Async,
{
    fn from_unbounded_sender<T>(sender: mpsc::UnboundedSender<T>) -> Self::Sender<T>
    where
        T: Async,
    {
        sender
    }

    fn from_unbounded_receiver<T>(receiver: mpsc::UnboundedReceiver<T>) -> Self::Receiver<T>
    where
        T: Async,
    {
        receiver
    }

    fn to_unbounded_receiver<T>(receiver: Self::Receiver<T>) -> mpsc::UnboundedReceiver<T>
    where
        T: Async,
    {
        receiver
    }
    fn to_unbounded_sender_ref<T>(sender: &Self::Sender<T>) -> &mpsc::UnboundedSender<T>
    where
        T: Async,
    {
        sender
    }

    fn to_unbounded_receiver_ref<T>(
        receiver: &mut Self::Receiver<T>,
    ) -> &mut mpsc::UnboundedReceiver<T>
    where
        T: Async,
    {
        receiver
    }
}

#[cgp_provider(ChannelCreatorComponent)]
impl<Runtime> ChannelCreator<Runtime> for ProvideUnboundedChannelType
where
    Runtime: HasUnboundedChannelType,
{
    fn new_channel<T>() -> (Runtime::Sender<T>, Runtime::Receiver<T>)
    where
        T: Async,
    {
        let (sender, receiver) = mpsc::unbounded_channel();

        (
            Runtime::from_unbounded_sender(sender),
            Runtime::from_unbounded_receiver(receiver),
        )
    }
}

#[cgp_provider(ChannelUserComponent)]
impl<Runtime> ChannelUser<Runtime> for ProvideUnboundedChannelType
where
    Runtime: HasUnboundedChannelType + CanRaiseAsyncError<ErrChannelClosed>,
{
    async fn send<T>(sender: &Runtime::Sender<T>, value: T) -> Result<(), Runtime::Error>
    where
        T: Async,
    {
        Runtime::to_unbounded_sender_ref(sender)
            .send(value)
            .map_err(|_| Runtime::raise_error(ErrChannelClosed))
    }

    async fn receive<T>(receiver: &mut Runtime::Receiver<T>) -> Result<T, Runtime::Error>
    where
        T: Async,
    {
        Runtime::to_unbounded_receiver_ref(receiver)
            .recv()
            .await
            .ok_or(Runtime::raise_error(ErrChannelClosed))
    }

    fn try_receive<T>(receiver: &mut Runtime::Receiver<T>) -> Result<Option<T>, Runtime::Error>
    where
        T: Async,
    {
        match Runtime::to_unbounded_receiver_ref(receiver).try_recv() {
            Ok(batch) => Ok(Some(batch)),
            Err(mpsc::error::TryRecvError::Empty) => Ok(None),
            Err(mpsc::error::TryRecvError::Disconnected) => {
                Err(Runtime::raise_error(ErrChannelClosed))
            }
        }
    }
}

#[cgp_provider(ReceiverStreamerComponent)]
impl<Runtime> ReceiverStreamer<Runtime> for ProvideUnboundedChannelType
where
    Runtime: HasUnboundedChannelType + HasBoxedStreamType,
{
    fn receiver_to_stream<T>(receiver: Runtime::Receiver<T>) -> Runtime::Stream<T>
    where
        T: Async,
    {
        Runtime::from_boxed_stream(Box::pin(UnboundedReceiverStream::new(
            Runtime::to_unbounded_receiver(receiver),
        )))
    }
}

#[cgp_provider(SenderClonerComponent)]
impl<Runtime> SenderCloner<Runtime> for ProvideUnboundedChannelType
where
    Runtime: HasUnboundedChannelType,
{
    fn clone_sender<T>(sender: &Runtime::Sender<T>) -> Runtime::Sender<T>
    where
        T: Async,
    {
        Runtime::from_unbounded_sender(Runtime::to_unbounded_sender_ref(sender).clone())
    }
}
