use alloc::vec::Vec;

use hermes_chain_type_components::traits::types::message_response::MessageResponseOf;
use hermes_relayer_components::chain::traits::types::message::HasMessageType;
use hermes_runtime_components::traits::channel::HasChannelTypes;
use hermes_runtime_components::traits::channel_once::HasChannelOnceTypes;
use hermes_runtime_components::traits::runtime::RuntimeOf;

pub type Sender<Chain, Payload> = <RuntimeOf<Chain> as HasChannelTypes>::Sender<Payload>;

pub type Receiver<Chain, Payload> = <RuntimeOf<Chain> as HasChannelTypes>::Receiver<Payload>;

pub type SenderOnce<Chain, Payload> =
    <RuntimeOf<Chain> as HasChannelOnceTypes>::SenderOnce<Payload>;

pub type ReceiverOnce<Chain, Payload> =
    <RuntimeOf<Chain> as HasChannelOnceTypes>::ReceiverOnce<Payload>;

pub type EventResult<Chain, Error> = Result<Vec<MessageResponseOf<Chain>>, Error>;

pub type EventResultSender<Chain, Error> = SenderOnce<Chain, EventResult<Chain, Error>>;

pub type EventResultReceiver<Chain, Error> = ReceiverOnce<Chain, EventResult<Chain, Error>>;

pub type BatchSubmission<Chain, Error> = (
    Vec<<Chain as HasMessageType>::Message>,
    EventResultSender<Chain, Error>,
);

pub type MessageBatchSender<Chain, Error> = Sender<Chain, BatchSubmission<Chain, Error>>;

pub type MessageBatchReceiver<Chain, Error> = Receiver<Chain, BatchSubmission<Chain, Error>>;
