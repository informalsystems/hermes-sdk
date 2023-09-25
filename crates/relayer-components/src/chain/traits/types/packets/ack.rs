use cgp_core::traits::sync::Async;

pub trait HasAckPacketPayload<Counterparty>: Async {
    type AckPacketPayload: Async;
}
