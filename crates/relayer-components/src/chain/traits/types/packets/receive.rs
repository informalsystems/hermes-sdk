use cgp_core::Async;

pub trait HasReceivePacketPayload<Counterparty>: Async {
    type ReceivePacketPayload: Async;
}
