use cgp_core::prelude::*;

#[derive_component(UpdateClientPayloadTypeComponent, ProvideUpdateClientPayloadType<Chain>)]
pub trait HasUpdateClientPayloadType<Counterparty>: Async {
    type UpdateClientPayload: Async;
}
