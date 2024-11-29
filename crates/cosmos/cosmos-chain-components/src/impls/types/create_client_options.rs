use cgp::prelude::Async;
use hermes_relayer_components::chain::traits::types::create_client::{
    ProvideCreateClientMessageOptionsType, ProvideCreateClientPayloadOptionsType,
};

use crate::types::payloads::client::CosmosCreateClientOptions;

pub struct ProvideCosmosCreateClientSettings;

impl<Chain, Counterparty> ProvideCreateClientPayloadOptionsType<Chain, Counterparty>
    for ProvideCosmosCreateClientSettings
where
    Chain: Async,
{
    type CreateClientPayloadOptions = CosmosCreateClientOptions;
}

pub struct ProvideNoCreateClientMessageOptionsType;

impl<Chain, Counterparty> ProvideCreateClientMessageOptionsType<Chain, Counterparty>
    for ProvideNoCreateClientMessageOptionsType
where
    Chain: Async,
{
    type CreateClientMessageOptions = ();
}
