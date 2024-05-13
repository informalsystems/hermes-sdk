use cgp_core::prelude::*;

#[derive_component(InitConnectionOptionsTypeComponent, ProvideInitConnectionOptionsType<Chain>)]
pub trait HasInitConnectionOptionsType<Counterparty>: Async {
    type InitConnectionOptions: Async;
}

pub type InitConnectionOptionsOf<Chain, Counterparty> =
    <Chain as HasInitConnectionOptionsType<Counterparty>>::InitConnectionOptions;

#[derive_component(ConnectionOpenInitPayloadTypeComponent, ProvideConnectionOpenInitPayloadType<Chain>)]
pub trait HasConnectionOpenInitPayloadType<Counterparty>: Async {
    type ConnectionOpenInitPayload: Async;
}

#[derive_component(ConnectionOpenTryPayloadTypeComponent, ProvideConnectionOpenTryPayloadType<Chain>)]
pub trait HasConnectionOpenTryPayloadType<Counterparty>: Async {
    type ConnectionOpenTryPayload: Async;
}

#[derive_component(ConnectionOpenAckPayloadTypeComponent, ProvideConnectionOpenAckPayloadType<Chain>)]
pub trait HasConnectionOpenAckPayloadType<Counterparty>: Async {
    type ConnectionOpenAckPayload: Async;
}

#[derive_component(ConnectionOpenConfirmPayloadTypeComponent, ProvideConnectionOpenConfirmPayloadType<Chain>)]
pub trait HasConnectionOpenConfirmPayloadType<Counterparty>: Async {
    type ConnectionOpenConfirmPayload: Async;
}

/**
    Payload that contains necessary counterparty information such as proofs and parameters
    in order for a self chain to build a connection handshake message.
*/
#[derive_component(ConnectionHandshakePayloadTypeComponent, ProvideConnectionHandshakePayloadTypes<Chain>)]
pub trait HasConnectionHandshakePayloadTypes<Counterparty>: Async {
    type ConnectionOpenInitPayload: Async;

    type ConnectionOpenTryPayload: Async;

    type ConnectionOpenAckPayload: Async;

    type ConnectionOpenConfirmPayload: Async;
}

#[derive_component(ConnectionEndTypeComponent, ProvideConnectionEndType<Chain>)]
pub trait HasConnectionEndType<Counterparty>: Async {
    type ConnectionEnd: Async;
}
