use cgp_core::Async;

pub trait HasAckPacketPayload<Counterparty>: Async {
    type AckPacketPayload: Async;
}
