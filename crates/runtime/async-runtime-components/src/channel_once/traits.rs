use cgp::prelude::*;
use futures_channel::oneshot::{Receiver, Sender};
use hermes_runtime_components::traits::channel_once::{
    ChannelOnceTypeComponent, HasChannelOnceTypes, ProvideChannelOnceType,
};

pub trait HasOneShotChannelType: HasChannelOnceTypes {
    fn from_oneshot_sender<T>(sender: Sender<T>) -> Self::SenderOnce<T>
    where
        T: Async;

    fn from_oneshot_receiver<T>(receiver: Receiver<T>) -> Self::ReceiverOnce<T>
    where
        T: Async;

    fn to_oneshot_sender<T>(sender: Self::SenderOnce<T>) -> Sender<T>
    where
        T: Async;

    fn to_oneshot_receiver<T>(receiver: Self::ReceiverOnce<T>) -> Receiver<T>
    where
        T: Async;
}

pub trait OneShotChannelTypeProvider<Runtime>: ProvideChannelOnceType<Runtime> {
    fn from_oneshot_sender<T>(sender: Sender<T>) -> Self::SenderOnce<T>
    where
        T: Async;

    fn from_oneshot_receiver<T>(receiver: Receiver<T>) -> Self::ReceiverOnce<T>
    where
        T: Async;

    fn to_oneshot_sender<T>(sender: Self::SenderOnce<T>) -> Sender<T>
    where
        T: Async;

    fn to_oneshot_receiver<T>(receiver: Self::ReceiverOnce<T>) -> Receiver<T>
    where
        T: Async;
}

impl<Runtime, Provider> HasOneShotChannelType for Runtime
where
    Runtime: HasProvider<Provider = Provider>,
    Provider: OneShotChannelTypeProvider<Runtime>,
{
    fn from_oneshot_sender<T>(sender: Sender<T>) -> Self::SenderOnce<T>
    where
        T: Async,
    {
        Provider::from_oneshot_sender(sender)
    }

    fn from_oneshot_receiver<T>(receiver: Receiver<T>) -> Self::ReceiverOnce<T>
    where
        T: Async,
    {
        Provider::from_oneshot_receiver(receiver)
    }

    fn to_oneshot_sender<T>(sender: Self::SenderOnce<T>) -> Sender<T>
    where
        T: Async,
    {
        Provider::to_oneshot_sender(sender)
    }

    fn to_oneshot_receiver<T>(receiver: Self::ReceiverOnce<T>) -> Receiver<T>
    where
        T: Async,
    {
        Provider::to_oneshot_receiver(receiver)
    }
}

impl<Runtime, Component, Delegate> OneShotChannelTypeProvider<Runtime> for Component
where
    Component: DelegateComponent<ChannelOnceTypeComponent, Delegate = Delegate>
        + IsProviderFor<ChannelOnceTypeComponent, Runtime, ()>,
    Delegate: OneShotChannelTypeProvider<Runtime>,
{
    fn from_oneshot_sender<T>(sender: Sender<T>) -> Self::SenderOnce<T>
    where
        T: Async,
    {
        Delegate::from_oneshot_sender(sender)
    }

    fn from_oneshot_receiver<T>(receiver: Receiver<T>) -> Self::ReceiverOnce<T>
    where
        T: Async,
    {
        Delegate::from_oneshot_receiver(receiver)
    }

    fn to_oneshot_sender<T>(sender: Self::SenderOnce<T>) -> Sender<T>
    where
        T: Async,
    {
        Delegate::to_oneshot_sender(sender)
    }

    fn to_oneshot_receiver<T>(receiver: Self::ReceiverOnce<T>) -> Receiver<T>
    where
        T: Async,
    {
        Delegate::to_oneshot_receiver(receiver)
    }
}
