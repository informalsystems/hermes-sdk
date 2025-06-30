use core::time::Duration;

use hermes_core::chain_components::traits::{
    HasCreateClientPayloadOptionsType, OverrideCreateClientPayloadOptionsComponent,
    ProvideOverrideCreateClientPayloadOptions,
};
use hermes_core::relayer_components::chain::traits::{
    CreateClientMessageOptionsTypeComponent, CreateClientPayloadOptionsTypeComponent,
    ProvideCreateClientMessageOptionsType, ProvideCreateClientPayloadOptionsType,
};
use hermes_prelude::*;

use crate::types::CosmosCreateClientOptions;

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

pub struct OverrideCosmosCreateClientPayloadOptions;

#[cgp_provider(OverrideCreateClientPayloadOptionsComponent)]
impl<Chain, Counterparty> ProvideOverrideCreateClientPayloadOptions<Chain, Counterparty>
    for OverrideCosmosCreateClientPayloadOptions
where
    Chain: HasCreateClientPayloadOptionsType<
        Counterparty,
        CreateClientPayloadOptions = CosmosCreateClientOptions,
    >,
{
    fn override_create_client_payload_options(
        payload_options: &CosmosCreateClientOptions,
        new_period: Duration,
    ) -> CosmosCreateClientOptions {
        CosmosCreateClientOptions {
            max_clock_drift: payload_options.max_clock_drift,
            trusting_period: new_period,
            trust_threshold: payload_options.trust_threshold,
        }
    }
}
