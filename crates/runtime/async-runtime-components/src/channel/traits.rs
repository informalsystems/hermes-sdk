use alloc::sync::Arc;

use cgp::prelude::*;
use futures_channel::mpsc;
use futures_util::lock::Mutex;
use hermes_runtime_components::traits::channel::{
    ChannelTypeComponent, HasChannelTypes, ProvideChannelType,
};

pub trait HasUnboundedChannelType: HasChannelTypes {
    fn from_unbounded_sender<T>(sender: Arc<Mutex<mpsc::UnboundedSender<T>>>) -> Self::Sender<T>
    where
        T: Async;

    fn from_unbounded_receiver<T>(receiver: mpsc::UnboundedReceiver<T>) -> Self::Receiver<T>
    where
        T: Async;

    fn to_unbounded_receiver<T>(receiver: Self::Receiver<T>) -> mpsc::UnboundedReceiver<T>
    where
        T: Async;

    fn to_unbounded_sender_ref<T>(
        sender: &Self::Sender<T>,
    ) -> &Arc<Mutex<mpsc::UnboundedSender<T>>>
    where
        T: Async;

    fn to_unbounded_receiver_ref<T>(
        receiver: &mut Self::Receiver<T>,
    ) -> &mut mpsc::UnboundedReceiver<T>
    where
        T: Async;
}

pub trait UnboundedChannelTypeProvider<Runtime>: ProvideChannelType<Runtime>
where
    Runtime: Async,
{
    fn from_unbounded_sender<T>(sender: Arc<Mutex<mpsc::UnboundedSender<T>>>) -> Self::Sender<T>
    where
        T: Async;

    fn from_unbounded_receiver<T>(receiver: mpsc::UnboundedReceiver<T>) -> Self::Receiver<T>
    where
        T: Async;

    fn to_unbounded_receiver<T>(receiver: Self::Receiver<T>) -> mpsc::UnboundedReceiver<T>
    where
        T: Async;

    fn to_unbounded_sender_ref<T>(
        sender: &Self::Sender<T>,
    ) -> &Arc<Mutex<mpsc::UnboundedSender<T>>>
    where
        T: Async;

    fn to_unbounded_receiver_ref<T>(
        receiver: &mut Self::Receiver<T>,
    ) -> &mut mpsc::UnboundedReceiver<T>
    where
        T: Async;
}

impl<Runtime, Provider> HasUnboundedChannelType for Runtime
where
    Runtime: Async + HasProvider<Provider = Provider>,
    Provider: UnboundedChannelTypeProvider<Runtime> + ProvideChannelType<Runtime>,
{
    fn from_unbounded_sender<T>(sender: Arc<Mutex<mpsc::UnboundedSender<T>>>) -> Self::Sender<T>
    where
        T: Async,
    {
        Provider::from_unbounded_sender(sender)
    }

    fn from_unbounded_receiver<T>(receiver: mpsc::UnboundedReceiver<T>) -> Self::Receiver<T>
    where
        T: Async,
    {
        Provider::from_unbounded_receiver(receiver)
    }

    fn to_unbounded_receiver<T>(receiver: Self::Receiver<T>) -> mpsc::UnboundedReceiver<T>
    where
        T: Async,
    {
        Provider::to_unbounded_receiver(receiver)
    }

    fn to_unbounded_sender_ref<T>(sender: &Self::Sender<T>) -> &Arc<Mutex<mpsc::UnboundedSender<T>>>
    where
        T: Async,
    {
        Provider::to_unbounded_sender_ref(sender)
    }

    fn to_unbounded_receiver_ref<T>(
        receiver: &mut Self::Receiver<T>,
    ) -> &mut mpsc::UnboundedReceiver<T>
    where
        T: Async,
    {
        Provider::to_unbounded_receiver_ref(receiver)
    }
}

impl<Runtime, Component, Delegate> UnboundedChannelTypeProvider<Runtime> for Component
where
    Runtime: Async,
    Component: DelegateComponent<ChannelTypeComponent, Delegate = Delegate>
        + IsProviderFor<ChannelTypeComponent, Runtime, ()>,
    Delegate: UnboundedChannelTypeProvider<Runtime>,
{
    fn from_unbounded_sender<T>(sender: Arc<Mutex<mpsc::UnboundedSender<T>>>) -> Self::Sender<T>
    where
        T: Async,
    {
        Delegate::from_unbounded_sender(sender)
    }

    fn from_unbounded_receiver<T>(receiver: mpsc::UnboundedReceiver<T>) -> Self::Receiver<T>
    where
        T: Async,
    {
        Delegate::from_unbounded_receiver(receiver)
    }

    fn to_unbounded_receiver<T>(receiver: Self::Receiver<T>) -> mpsc::UnboundedReceiver<T>
    where
        T: Async,
    {
        Delegate::to_unbounded_receiver(receiver)
    }

    fn to_unbounded_sender_ref<T>(sender: &Self::Sender<T>) -> &Arc<Mutex<mpsc::UnboundedSender<T>>>
    where
        T: Async,
    {
        Delegate::to_unbounded_sender_ref(sender)
    }

    fn to_unbounded_receiver_ref<T>(
        receiver: &mut Self::Receiver<T>,
    ) -> &mut mpsc::UnboundedReceiver<T>
    where
        T: Async,
    {
        Delegate::to_unbounded_receiver_ref(receiver)
    }
}
