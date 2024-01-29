use cgp_core::Async;
use hermes_relayer_components::chain::traits::types::channel::ProvideChannelHandshakePayloadTypes;
use hermes_relayer_components::chain::traits::types::connection::ProvideConnectionHandshakePayloadTypes;
use hermes_relayer_components::chain::traits::types::packets::ack::ProvideAckPacketPayloadType;
use hermes_relayer_components::chain::traits::types::packets::receive::ProvideReceivePacketPayloadType;
use hermes_relayer_components::chain::traits::types::packets::timeout::ProvideTimeoutUnorderedPacketPayloadType;

use crate::types::payloads::channel::{
    CosmosChannelOpenAckPayload, CosmosChannelOpenConfirmPayload, CosmosChannelOpenTryPayload,
};
use crate::types::payloads::connection::{
    CosmosConnectionOpenAckPayload, CosmosConnectionOpenConfirmPayload,
    CosmosConnectionOpenInitPayload, CosmosConnectionOpenTryPayload,
};
use crate::types::payloads::packet::{
    CosmosAckPacketPayload, CosmosReceivePacketPayload, CosmosTimeoutUnorderedPacketPayload,
};

pub struct ProvideCosmosPayloadTypes;

impl<Chain, Counterparty> ProvideChannelHandshakePayloadTypes<Chain, Counterparty>
    for ProvideCosmosPayloadTypes
where
    Chain: Async,
{
    type ChannelOpenTryPayload = CosmosChannelOpenTryPayload;

    type ChannelOpenAckPayload = CosmosChannelOpenAckPayload;

    type ChannelOpenConfirmPayload = CosmosChannelOpenConfirmPayload;
}

impl<Chain, Counterparty> ProvideConnectionHandshakePayloadTypes<Chain, Counterparty>
    for ProvideCosmosPayloadTypes
where
    Chain: Async,
{
    type ConnectionOpenInitPayload = CosmosConnectionOpenInitPayload;

    type ConnectionOpenTryPayload = CosmosConnectionOpenTryPayload;

    type ConnectionOpenAckPayload = CosmosConnectionOpenAckPayload;

    type ConnectionOpenConfirmPayload = CosmosConnectionOpenConfirmPayload;
}

impl<Chain, Counterparty> ProvideReceivePacketPayloadType<Chain, Counterparty>
    for ProvideCosmosPayloadTypes
where
    Chain: Async,
{
    type ReceivePacketPayload = CosmosReceivePacketPayload;
}

impl<Chain, Counterparty> ProvideAckPacketPayloadType<Chain, Counterparty>
    for ProvideCosmosPayloadTypes
where
    Chain: Async,
{
    type AckPacketPayload = CosmosAckPacketPayload;
}

impl<Chain, Counterparty> ProvideTimeoutUnorderedPacketPayloadType<Chain, Counterparty>
    for ProvideCosmosPayloadTypes
where
    Chain: Async,
{
    type TimeoutUnorderedPacketPayload = CosmosTimeoutUnorderedPacketPayload;
}
