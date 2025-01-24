/*!
   Trait definitions for [`HasWriteAckEvent`].
*/

use cgp::prelude::*;

/**
   Indicates that a chain context's
   [`Event`](crate::traits::types::event::HasEventType::Event)
   type contains a [`WriteAckEvent`](Self::WriteAckEvent) variant.
*/
#[cgp_component {
  name: WriteAckEventComponent,
  provider: ProvideWriteAckEvent,
  context: Chain,
}]
pub trait HasWriteAckEvent<Counterparty> {
    /**
       The write acknowledgement event that is emitted when a `RecvPacket`
       message is committed to a chain.

       At the moment, there is no need for the relayer framework to know
       further information about the write acknowledgement event, other
       than passing it down to the concrete context to build the `Ack`
       message.

       If new components have the need to extract information out of
       the write acknowledgement event, such as the ack payload,
       we can add new methods to this trait to do the extraction.
    */
    type WriteAckEvent: Async;
}
