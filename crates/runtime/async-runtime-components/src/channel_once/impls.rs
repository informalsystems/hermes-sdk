use cgp::prelude::*;
use futures_channel::oneshot::{channel, Receiver, Sender};
use hermes_runtime_components::traits::channel_once::{
    ChannelOnceCreator, ChannelOnceUser, ProvideChannelOnceType,
};

use crate::channel::types::ErrChannelClosed;
use crate::channel_once::traits::{HasOneShotChannelType, OneShotChannelTypeProvider};

pub struct ProvideOneShotChannelType;

impl<Runtime> ProvideChannelOnceType<Runtime> for ProvideOneShotChannelType
where
    Runtime: Async,
{
    type SenderOnce<T>
        = Sender<T>
    where
        T: Async;

    type ReceiverOnce<T>
        = Receiver<T>
    where
        T: Async;
}

impl<Runtime> OneShotChannelTypeProvider<Runtime> for ProvideOneShotChannelType
where
    Runtime: Async,
{
    fn from_oneshot_sender<T>(sender: Sender<T>) -> Self::SenderOnce<T>
    where
        T: Async,
    {
        sender
    }

    fn from_oneshot_receiver<T>(receiver: Receiver<T>) -> Self::ReceiverOnce<T>
    where
        T: Async,
    {
        receiver
    }

    fn to_oneshot_sender<T>(sender: Self::SenderOnce<T>) -> Sender<T>
    where
        T: Async,
    {
        sender
    }

    fn to_oneshot_receiver<T>(receiver: Self::ReceiverOnce<T>) -> Receiver<T>
    where
        T: Async,
    {
        receiver
    }
}

impl<Runtime> ChannelOnceCreator<Runtime> for ProvideOneShotChannelType
where
    Runtime: HasOneShotChannelType,
{
    fn new_channel_once<T>() -> (Runtime::SenderOnce<T>, Runtime::ReceiverOnce<T>)
    where
        T: Async,
    {
        let (sender, receiver) = channel();

        (
            Runtime::from_oneshot_sender(sender),
            Runtime::from_oneshot_receiver(receiver),
        )
    }
}

impl<Runtime> ChannelOnceUser<Runtime> for ProvideOneShotChannelType
where
    Runtime: HasOneShotChannelType + CanRaiseAsyncError<ErrChannelClosed>,
{
    fn send_once<T>(sender: Runtime::SenderOnce<T>, value: T) -> Result<(), Runtime::Error>
    where
        T: Async,
    {
        Runtime::to_oneshot_sender(sender)
            .send(value)
            .map_err(|_| Runtime::raise_error(ErrChannelClosed))
    }

    async fn receive_once<T>(receiver: Runtime::ReceiverOnce<T>) -> Result<T, Runtime::Error>
    where
        T: Async,
    {
        Runtime::to_oneshot_receiver(receiver)
            .await
            .map_err(|_| Runtime::raise_error(ErrChannelClosed))
    }
}
