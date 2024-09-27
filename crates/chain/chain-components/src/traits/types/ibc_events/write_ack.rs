/*!
   Trait definitions for [`HasWriteAckEvent`].
*/

use cgp::prelude::*;

use crate::traits::types::event::HasEventType;
use crate::traits::types::packets::ack::HasAcknowledgementType;

/**
   Indicates that a chain context's
   [`Event`](crate::traits::types::event::HasEventType::Event)
   type contains a [`WriteAckEvent`](Self::WriteAckEvent) variant.
*/
#[derive_component(WriteAckEventComponent, ProvideWriteAckEvent<Chain>)]
pub trait HasWriteAckEvent<Counterparty>:
    HasEventType + HasAcknowledgementType<Counterparty>
{
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

    /**
       Try to extract an abstract
       [`Event`](crate::traits::types::event::HasEventType::Event)
       type into a
       [`WriteAckEvent`](Self::WriteAckEvent).
       If the extraction fails, return `None`.

       Since an event type may contain many variants, it is not guaranteed
       that the event extraction would be successful. If the concrete
       `Event` is dynamic-typed, then the extraction may also fail due to
       parse errors.
    */
    fn try_extract_write_ack_event(event: &Self::Event) -> Option<Self::WriteAckEvent>;

    fn write_acknowledgement(
        event: &Self::WriteAckEvent,
    ) -> impl AsRef<Self::Acknowledgement> + Send;
}
