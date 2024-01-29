use cgp_core::prelude::*;

#[derive_component(InitConnectionOptionsTypeComponent, ProvideInitConnectionOptionsType<Chain>)]
pub trait HasInitConnectionOptionsType<Counterparty>: Async {
    type InitConnectionOptions: Async;
}

pub type InitConnectionOptionsOf<Chain, Counterparty> =
    <Chain as HasInitConnectionOptionsType<Counterparty>>::InitConnectionOptions;

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
