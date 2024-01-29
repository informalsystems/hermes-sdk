use cgp_core::Async;
use hermes_relayer_components::chain::traits::types::connection::ProvideConnectionHandshakePayloadTypes;

use crate::types::payloads::connection::{
    CosmosConnectionOpenAckPayload, CosmosConnectionOpenConfirmPayload,
    CosmosConnectionOpenInitPayload, CosmosConnectionOpenTryPayload,
};

pub struct ProvideCosmosConnectionHandshakePayloads;

impl<Chain, Counterparty> ProvideConnectionHandshakePayloadTypes<Chain, Counterparty>
    for ProvideCosmosConnectionHandshakePayloads
where
    Chain: Async,
{
    type ConnectionOpenInitPayload = CosmosConnectionOpenInitPayload;

    type ConnectionOpenTryPayload = CosmosConnectionOpenTryPayload;

    type ConnectionOpenAckPayload = CosmosConnectionOpenAckPayload;

    type ConnectionOpenConfirmPayload = CosmosConnectionOpenConfirmPayload;
}
