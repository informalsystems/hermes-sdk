use cgp_core::traits::Async;

pub trait HasAckPacketPayload<Counterparty>: Async {
    type AckPacketPayload: Async;
}
