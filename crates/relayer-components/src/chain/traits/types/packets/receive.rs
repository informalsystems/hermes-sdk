use cgp_core::traits::Async;

pub trait HasReceivePacketPayload<Counterparty>: Async {
    type ReceivePacketPayload: Async;
}
