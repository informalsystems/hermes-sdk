use cgp_core::prelude::Async;
use ibc_relayer_components_extra::runtime::traits::channel::ProvideChannelType;
use tokio::sync::mpsc;

pub struct ProvideUnboundedChannelType;

impl<Runtime> ProvideChannelType<Runtime> for ProvideUnboundedChannelType
where
    Runtime: Async,
{
    type Sender<T> = mpsc::UnboundedSender<T>
    where
        T: Async;

    type Receiver<T> = mpsc::UnboundedReceiver<T>
    where
        T: Async;
}
