use cgp::prelude::*;
use hermes_runtime_components::traits::channel::{
    ChannelTypeComponent, HasChannelTypes, ProvideChannelType,
};
use tokio::sync::mpsc;

pub trait HasUnboundedChannelType: HasChannelTypes {
    fn from_unbounded_sender<T>(sender: mpsc::UnboundedSender<T>) -> Self::Sender<T>
    where
        T: Async;

    fn from_unbounded_receiver<T>(receiver: mpsc::UnboundedReceiver<T>) -> Self::Receiver<T>
    where
        T: Async;

    fn to_unbounded_receiver<T>(receiver: Self::Receiver<T>) -> mpsc::UnboundedReceiver<T>
    where
        T: Async;

    fn to_unbounded_sender_ref<T>(sender: &Self::Sender<T>) -> &mpsc::UnboundedSender<T>
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
    fn from_unbounded_sender<T>(sender: mpsc::UnboundedSender<T>) -> Self::Sender<T>
    where
        T: Async;

    fn from_unbounded_receiver<T>(receiver: mpsc::UnboundedReceiver<T>) -> Self::Receiver<T>
    where
        T: Async;

    fn to_unbounded_receiver<T>(receiver: Self::Receiver<T>) -> mpsc::UnboundedReceiver<T>
    where
        T: Async;

    fn to_unbounded_sender_ref<T>(sender: &Self::Sender<T>) -> &mpsc::UnboundedSender<T>
    where
        T: Async;

    fn to_unbounded_receiver_ref<T>(
        receiver: &mut Self::Receiver<T>,
    ) -> &mut mpsc::UnboundedReceiver<T>
    where
        T: Async;
}

impl<Runtime, Components> HasUnboundedChannelType for Runtime
where
    Runtime: Async + HasComponents<Components = Components>,
    Components: UnboundedChannelTypeProvider<Runtime> + ProvideChannelType<Runtime>,
{
    fn from_unbounded_sender<T>(sender: mpsc::UnboundedSender<T>) -> Self::Sender<T>
    where
        T: Async,
    {
        Components::from_unbounded_sender(sender)
    }

    fn from_unbounded_receiver<T>(receiver: mpsc::UnboundedReceiver<T>) -> Self::Receiver<T>
    where
        T: Async,
    {
        Components::from_unbounded_receiver(receiver)
    }

    fn to_unbounded_receiver<T>(receiver: Self::Receiver<T>) -> mpsc::UnboundedReceiver<T>
    where
        T: Async,
    {
        Components::to_unbounded_receiver(receiver)
    }

    fn to_unbounded_sender_ref<T>(sender: &Self::Sender<T>) -> &mpsc::UnboundedSender<T>
    where
        T: Async,
    {
        Components::to_unbounded_sender_ref(sender)
    }

    fn to_unbounded_receiver_ref<T>(
        receiver: &mut Self::Receiver<T>,
    ) -> &mut mpsc::UnboundedReceiver<T>
    where
        T: Async,
    {
        Components::to_unbounded_receiver_ref(receiver)
    }
}

impl<Runtime, Component, Delegate> UnboundedChannelTypeProvider<Runtime> for Component
where
    Runtime: Async,
    Component: DelegateComponent<ChannelTypeComponent, Delegate = Delegate>
        + IsProviderFor<ChannelTypeComponent, Runtime, ()>,
    Delegate: UnboundedChannelTypeProvider<Runtime>,
{
    fn from_unbounded_sender<T>(sender: mpsc::UnboundedSender<T>) -> Self::Sender<T>
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

    fn to_unbounded_sender_ref<T>(sender: &Self::Sender<T>) -> &mpsc::UnboundedSender<T>
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
