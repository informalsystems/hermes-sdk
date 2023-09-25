use cgp_core::traits::Async;

pub trait HasTimeoutUnorderedPacketPayload<Counterparty>: Async {
    type TimeoutUnorderedPacketPayload: Async;
}
