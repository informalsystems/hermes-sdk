use cgp_async::async_trait;
use cgp_core::traits::{Async, HasErrorType};

use crate::std_prelude::*;

pub trait HasChannelOnceTypes: HasErrorType {
    type SenderOnce<T>: Async
    where
        T: Async;

    type ReceiverOnce<T>: Async
    where
        T: Async;
}

pub trait CanCreateChannelsOnce: HasChannelOnceTypes {
    fn new_channel_once<T>() -> (Self::SenderOnce<T>, Self::ReceiverOnce<T>)
    where
        T: Async;
}

#[async_trait]
pub trait CanUseChannelsOnce: HasChannelOnceTypes {
    fn send_once<T>(sender: Self::SenderOnce<T>, value: T) -> Result<(), Self::Error>
    where
        T: Async;

    async fn receive_once<T>(receiver: Self::ReceiverOnce<T>) -> Result<T, Self::Error>
    where
        T: Async;
}
