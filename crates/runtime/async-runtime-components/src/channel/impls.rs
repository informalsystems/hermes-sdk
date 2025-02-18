use alloc::boxed::Box;
use alloc::sync::Arc;

use cgp::prelude::*;
use futures_channel::mpsc;
use futures_util::lock::Mutex;
use futures_util::stream::StreamExt;
use hermes_runtime_components::traits::channel::{
    ChannelCreator, ChannelCreatorComponent, ChannelTypeComponent, ChannelUser,
    ChannelUserComponent, ProvideChannelType, ReceiverStreamer, ReceiverStreamerComponent,
    SenderCloner, SenderClonerComponent,
};

use crate::channel::traits::{HasUnboundedChannelType, UnboundedChannelTypeProvider};
use crate::channel::types::ErrChannelClosed;
use crate::stream::traits::boxed::HasBoxedStreamType;

pub struct ProvideUnboundedChannelType;

#[cgp_provider(ChannelTypeComponent)]
impl<Runtime> ProvideChannelType<Runtime> for ProvideUnboundedChannelType
where
    Runtime: Async,
{
    type Sender<T>
        = Arc<Mutex<mpsc::UnboundedSender<T>>>
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
    fn from_unbounded_sender<T>(sender: Arc<Mutex<mpsc::UnboundedSender<T>>>) -> Self::Sender<T>
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
    fn to_unbounded_sender_ref<T>(sender: &Self::Sender<T>) -> &Arc<Mutex<mpsc::UnboundedSender<T>>>
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
        let (sender, receiver) = mpsc::unbounded();

        (
            Runtime::from_unbounded_sender(Arc::new(Mutex::new(sender))),
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
            .lock()
            .await
            .unbounded_send(value)
            .map_err(|_| Runtime::raise_error(ErrChannelClosed))
    }

    async fn receive<T>(receiver: &mut Runtime::Receiver<T>) -> Result<T, Runtime::Error>
    where
        T: Async,
    {
        Runtime::to_unbounded_receiver_ref(receiver)
            .next()
            .await
            .ok_or(Runtime::raise_error(ErrChannelClosed))
    }

    fn try_receive<T>(receiver: &mut Runtime::Receiver<T>) -> Result<Option<T>, Runtime::Error>
    where
        T: Async,
    {
        let res = Runtime::to_unbounded_receiver_ref(receiver).try_next();

        // The result semantics of the futures version of receiver is slightly different
        match res {
            Ok(Some(res)) => Ok(Some(res)),
            // Ok(None) means that the channel is closed
            Ok(None) => Err(Runtime::raise_error(ErrChannelClosed)),
            // Error means that there is no meesage currently available
            Err(_) => Ok(None),
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
        Runtime::from_boxed_stream(Box::pin(Runtime::to_unbounded_receiver(receiver)))
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
