use alloc::vec::Vec;

use hermes_relayer_components::chain::traits::types::event::HasEventType;
use hermes_relayer_components::chain::traits::types::message::HasMessageType;
use hermes_relayer_components::runtime::types::aliases::Runtime;

use crate::runtime::traits::channel::HasChannelTypes;
use crate::runtime::traits::channel_once::HasChannelOnceTypes;

pub type Sender<Chain, Payload> = <Runtime<Chain> as HasChannelTypes>::Sender<Payload>;

pub type Receiver<Chain, Payload> = <Runtime<Chain> as HasChannelTypes>::Receiver<Payload>;

pub type SenderOnce<Chain, Payload> = <Runtime<Chain> as HasChannelOnceTypes>::SenderOnce<Payload>;

pub type ReceiverOnce<Chain, Payload> =
    <Runtime<Chain> as HasChannelOnceTypes>::ReceiverOnce<Payload>;

pub type EventResult<Chain, Error> = Result<Vec<Vec<<Chain as HasEventType>::Event>>, Error>;

pub type EventResultSender<Chain, Error> = SenderOnce<Chain, EventResult<Chain, Error>>;

pub type EventResultReceiver<Chain, Error> = ReceiverOnce<Chain, EventResult<Chain, Error>>;

pub type BatchSubmission<Chain, Error> = (
    Vec<<Chain as HasMessageType>::Message>,
    EventResultSender<Chain, Error>,
);

pub type MessageBatchSender<Chain, Error> = Sender<Chain, BatchSubmission<Chain, Error>>;

pub type MessageBatchReceiver<Chain, Error> = Receiver<Chain, BatchSubmission<Chain, Error>>;
