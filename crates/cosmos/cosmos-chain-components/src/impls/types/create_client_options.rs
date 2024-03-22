use cgp_core::prelude::Async;
use hermes_relayer_components::chain::traits::types::create_client::ProvideCreateClientOptionsType;
use ibc_relayer::chain::client::ClientSettings;

pub struct ProvideCosmosCreateClientSettings;

impl<Chain, Counterparty> ProvideCreateClientOptionsType<Chain, Counterparty>
    for ProvideCosmosCreateClientSettings
where
    Chain: Async,
    Counterparty: Async,
{
    type CreateClientOptions = ClientSettings;
}
