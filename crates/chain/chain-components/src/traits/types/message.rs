/*!
   Trait definitions for [`HasMessageType`] and [`CanEstimateMessageSize`].
*/

use cgp::prelude::*;
pub use hermes_chain_type_components::traits::types::message::*;

#[cgp_component {
  name: MessageSizeEstimatorComponent,
  provider: MessageSizeEstimator,
  context: Chain,
}]
pub trait CanEstimateMessageSize: HasMessageType + HasErrorType {
    /**
       Estimate the size of a message after it is encoded into raw bytes
       inside a transaction.

       Because the signer field of a message is late-bound, it may not
       be possible to get a precise size if the signer field can have
       dynamic length. For the purpose of length estimation, the concrete
       context may replace the message's signer field with a dummy signer
       value, so that it can be encoded into raw bytes.

       This is mainly used by the `BatchMessageWorker` to estimate the
       the message size when batching messages. We may consider moving
       this method into a separate crate if it is not being used elsewhere.
    */
    fn estimate_message_size(message: &Self::Message) -> Result<usize, Self::Error>;
}
