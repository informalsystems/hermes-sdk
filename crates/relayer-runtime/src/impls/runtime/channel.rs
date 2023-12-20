use core::pin::Pin;

use async_trait::async_trait;
use cgp_core::Async;
use futures::Stream;
use ibc_relayer_components_extra::runtime::traits::channel::{CanCloneSender, CanStreamReceiver};
use ibc_relayer_components_extra::runtime::traits::channel_once::{
    CanCreateChannelsOnce, CanUseChannelsOnce, HasChannelOnceTypes,
};
use tokio::sync::oneshot;
use tokio_stream::wrappers::UnboundedReceiverStream;

use crate::types::error::TokioRuntimeError;
use crate::types::runtime::TokioRuntimeContext;

impl HasChannelOnceTypes for TokioRuntimeContext {
    type SenderOnce<T> = oneshot::Sender<T>
    where
        T: Async;

    type ReceiverOnce<T> = oneshot::Receiver<T>
    where
        T: Async;
}

impl CanCreateChannelsOnce for TokioRuntimeContext {
    fn new_channel_once<T>() -> (Self::SenderOnce<T>, Self::ReceiverOnce<T>)
    where
        T: Async,
    {
        let (sender, receiver) = oneshot::channel();
        (sender, receiver)
    }
}

#[async_trait]
impl CanUseChannelsOnce for TokioRuntimeContext {
    fn send_once<T>(sender: Self::SenderOnce<T>, value: T) -> Result<(), Self::Error>
    where
        T: Async,
    {
        sender
            .send(value)
            .map_err(|_| TokioRuntimeError::ChannelClosed)
    }

    async fn receive_once<T>(receiver: Self::ReceiverOnce<T>) -> Result<T, Self::Error>
    where
        T: Async,
    {
        receiver.await.map_err(|_| TokioRuntimeError::ChannelClosed)
    }
}

impl CanStreamReceiver for TokioRuntimeContext {
    fn receiver_to_stream<T>(
        receiver: Self::Receiver<T>,
    ) -> Pin<Box<dyn Stream<Item = T> + Send + Sync + 'static>>
    where
        T: Async,
    {
        Box::pin(UnboundedReceiverStream::new(receiver))
    }
}

impl CanCloneSender for TokioRuntimeContext {
    fn clone_sender<T>(sender: &Self::Sender<T>) -> Self::Sender<T>
    where
        T: Async,
    {
        sender.clone()
    }
}
