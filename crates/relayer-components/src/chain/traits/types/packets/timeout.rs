use cgp_core::traits::sync::Async;

pub trait HasTimeoutUnorderedPacketPayload<Counterparty>: Async {
    type TimeoutUnorderedPacketPayload: Async;
}
