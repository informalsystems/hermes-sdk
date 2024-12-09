use cgp::prelude::*;

#[cgp_component {
  name: ChannelOnceTypeComponent,
  provider: ProvideChannelOnceType,
  context: Runtime,
}]
pub trait HasChannelOnceTypes {
    type SenderOnce<T>: Async
    where
        T: Async;

    type ReceiverOnce<T>: Async
    where
        T: Async;
}

pub type SenderOnceOf<Runtime, T> = <Runtime as HasChannelOnceTypes>::SenderOnce<T>;

pub type ReceiverOnce<Runtime, T> = <Runtime as HasChannelOnceTypes>::ReceiverOnce<T>;

#[cgp_component {
  provider: ChannelOnceCreator,
  context: Runtime,
}]
pub trait CanCreateChannelsOnce: HasChannelOnceTypes {
    fn new_channel_once<T>() -> (Self::SenderOnce<T>, Self::ReceiverOnce<T>)
    where
        T: Async;
}

#[cgp_component {
  provider: ChannelOnceUser,
  context: Runtime,
}]
#[async_trait]
pub trait CanUseChannelsOnce: HasChannelOnceTypes + HasErrorType {
    fn send_once<T>(sender: Self::SenderOnce<T>, value: T) -> Result<(), Self::Error>
    where
        T: Async;

    async fn receive_once<T>(receiver: Self::ReceiverOnce<T>) -> Result<T, Self::Error>
    where
        T: Async;
}
