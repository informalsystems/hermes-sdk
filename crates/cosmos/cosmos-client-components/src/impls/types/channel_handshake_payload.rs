use cgp_core::Async;
use hermes_relayer_components::chain::traits::types::channel::ProvideChannelHandshakePayloadTypes;

use crate::types::payloads::channel::{
    CosmosChannelOpenAckPayload, CosmosChannelOpenConfirmPayload, CosmosChannelOpenTryPayload,
};

pub struct ProvideCosmosChannelHandshakePayloads;

impl<Chain, Counterparty> ProvideChannelHandshakePayloadTypes<Chain, Counterparty>
    for ProvideCosmosChannelHandshakePayloads
where
    Chain: Async,
{
    type ChannelOpenTryPayload = CosmosChannelOpenTryPayload;

    type ChannelOpenAckPayload = CosmosChannelOpenAckPayload;

    type ChannelOpenConfirmPayload = CosmosChannelOpenConfirmPayload;
}
