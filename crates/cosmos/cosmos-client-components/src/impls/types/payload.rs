use cgp_core::Async;
use hermes_relayer_components::chain::traits::types::packets::receive::ProvideReceivePacketPayloadType;

use crate::types::payloads::packet::CosmosReceivePacketPayload;

pub struct ProvideCosmosPayloadTypes;

impl<Chain, Counterparty> ProvideReceivePacketPayloadType<Chain, Counterparty>
    for ProvideCosmosPayloadTypes
where
    Chain: Async,
{
    type ReceivePacketPayload = CosmosReceivePacketPayload;
}
