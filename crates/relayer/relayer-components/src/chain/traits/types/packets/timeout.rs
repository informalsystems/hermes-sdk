use cgp_core::Async;

pub trait HasTimeoutUnorderedPacketPayload<Counterparty>: Async {
    type TimeoutUnorderedPacketPayload: Async;
}
