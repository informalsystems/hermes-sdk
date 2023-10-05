/*!
   Trait definitions for [`HasWriteAckEvent`].
*/

use cgp_core::Async;

use crate::chain::traits::types::event::HasEventType;
use crate::chain::traits::types::ibc::HasIbcChainTypes;
use crate::chain::traits::types::packet::HasIbcPacketTypes;

/**
   Indicates that a chain context's
   [`Event`](crate::chain::traits::types::event::HasEventType::Event)
   type contains a [`WriteAckEvent`](Self::WriteAckEvent) variant.
*/
pub trait HasWriteAckEvent<Counterparty>: HasEventType {
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
       [`Event`](crate::chain::traits::types::event::HasEventType::Event)
       type into a
       [`WriteAckEvent`](Self::WriteAckEvent).
       If the extraction fails, return `None`.

       Since an event type may contain many variants, it is not guaranteed
       that the event extraction would be successful. If the concrete
       `Event` is dynamic-typed, then the extraction may also fail due to
       parse errors.
    */
    fn try_extract_write_ack_event(event: &Self::Event) -> Option<Self::WriteAckEvent>;
}

pub trait CanBuildPacketFromWriteAckEvent<Counterparty>:
    HasWriteAckEvent<Counterparty> + HasIbcPacketTypes<Counterparty>
where
    Counterparty: HasIbcChainTypes<Self>,
{
    /**
       Extract the [`IncomingPacket`](HasIbcPacketTypes::IncomingPacket)
       from a write acknowledgement event.

       Since write acknowledgements are emitted from a destination chain (self),
       it is necessary for the event to correspond to an incoming packet
       (with self being the destination).

       Here we assume that a write acknowledgement event always contains
       the packet data. This is currently true for Cosmos chains. However
       in case additional queries are required, then this method should be
       refactored into a method like
       `query_packet_from_write_ack_event`.
    */
    fn build_packet_from_write_ack_event(ack: &Self::WriteAckEvent) -> &Self::IncomingPacket;
}
