use cgp_core::prelude::*;

#[derive_component(TimeoutUnorderedPacketPayloadTypeComponent, ProvideTimeoutUnorderedPacketPayloadType<Chain>)]
pub trait HasTimeoutUnorderedPacketPayloadType<Counterparty>: Async {
    type TimeoutUnorderedPacketPayload: Async;
}
