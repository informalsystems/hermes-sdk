use cgp_core::prelude::Async;
use hermes_relayer_components::chain::traits::types::create_client::{
    ProvideCreateClientMessageOptionsType, ProvideCreateClientPayloadOptionsType,
};
use ibc_relayer::chain::client::ClientSettings;

pub struct ProvideCosmosCreateClientSettings;

impl<Chain, Counterparty> ProvideCreateClientPayloadOptionsType<Chain, Counterparty>
    for ProvideCosmosCreateClientSettings
where
    Chain: Async,
    Counterparty: Async,
{
    type CreateClientPayloadOptions = ClientSettings;
}

impl<Chain, Counterparty> ProvideCreateClientMessageOptionsType<Chain, Counterparty>
    for ProvideCosmosCreateClientSettings
where
    Chain: Async,
    Counterparty: Async,
{
    type CreateClientMessageOptions = ();
}
