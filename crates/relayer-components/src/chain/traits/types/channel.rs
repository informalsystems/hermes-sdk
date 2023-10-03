use cgp_core::Async;

use crate::chain::traits::types::ibc::HasIbcChainTypes;

pub trait HasInitChannelOptionsType<Counterparty>: HasIbcChainTypes<Counterparty> {
    type InitChannelOptions: Async;
}

/**
    Payload that contains necessary counterparty information such as proofs and parameters
    in order for a self chain to build a channel handshake message.
*/
pub trait HasChannelHandshakePayloads<Counterparty>: HasIbcChainTypes<Counterparty> {
    type ChannelOpenTryPayload: Async;

    type ChannelOpenAckPayload: Async;

    type ChannelOpenConfirmPayload: Async;
}
