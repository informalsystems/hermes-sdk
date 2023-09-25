use cgp_core::traits::sync::Async;

pub trait HasReceivePacketPayload<Counterparty>: Async {
    type ReceivePacketPayload: Async;
}
