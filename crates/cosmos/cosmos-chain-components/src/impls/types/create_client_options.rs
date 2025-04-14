use cgp::prelude::*;
use hermes_relayer_components::chain::traits::{
    CreateClientMessageOptionsTypeComponent, CreateClientPayloadOptionsTypeComponent,
    ProvideCreateClientMessageOptionsType, ProvideCreateClientPayloadOptionsType,
};

use crate::types::payloads::client::CosmosCreateClientOptions;

pub struct ProvideCosmosCreateClientSettings;

#[cgp_provider(CreateClientPayloadOptionsTypeComponent)]
impl<Chain, Counterparty> ProvideCreateClientPayloadOptionsType<Chain, Counterparty>
    for ProvideCosmosCreateClientSettings
where
    Chain: Async,
{
    type CreateClientPayloadOptions = CosmosCreateClientOptions;
}

pub struct ProvideNoCreateClientMessageOptionsType;

#[cgp_provider(CreateClientMessageOptionsTypeComponent)]
impl<Chain, Counterparty> ProvideCreateClientMessageOptionsType<Chain, Counterparty>
    for ProvideNoCreateClientMessageOptionsType
where
    Chain: Async,
{
    type CreateClientMessageOptions = ();
}
