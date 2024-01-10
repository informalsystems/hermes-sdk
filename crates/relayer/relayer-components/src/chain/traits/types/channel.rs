use cgp_core::Async;

pub trait HasInitChannelOptionsType<Counterparty> {
    type InitChannelOptions: Async;
}

pub type InitChannelOptions<Chain, Counterparty> =
    <Chain as HasInitChannelOptionsType<Counterparty>>::InitChannelOptions;

/**
    Payload that contains necessary counterparty information such as proofs and parameters
    in order for a self chain to build a channel handshake message.
*/
pub trait HasChannelHandshakePayloads<Counterparty> {
    type ChannelOpenTryPayload: Async;

    type ChannelOpenAckPayload: Async;

    type ChannelOpenConfirmPayload: Async;
}
