use cgp_core::prelude::Async;
use hermes_relayer_components::chain::traits::types::channel::ProvideInitChannelOptionsType;
use hermes_relayer_components::chain::traits::types::connection::ProvideInitConnectionOptionsType;
use hermes_relayer_components::chain::traits::types::create_client::ProvideCreateClientOptionsType;
use ibc_relayer::chain::client::ClientSettings;

use crate::types::payloads::channel::SovereignInitChannelOptions;
use crate::types::payloads::connection::SovereignInitConnectionOptions;

pub struct ProvideSovereignRollupPayloadTypes;

impl<Chain, Counterparty> ProvideCreateClientOptionsType<Chain, Counterparty>
    for ProvideSovereignRollupPayloadTypes
where
    Chain: Async,
{
    type CreateClientOptions = ClientSettings;
}

impl<Chain, Counterparty> ProvideInitConnectionOptionsType<Chain, Counterparty>
    for ProvideSovereignRollupPayloadTypes
where
    Chain: Async,
{
    type InitConnectionOptions = SovereignInitConnectionOptions;
}

impl<Chain, Counterparty> ProvideInitChannelOptionsType<Chain, Counterparty>
    for ProvideSovereignRollupPayloadTypes
where
    Chain: Async,
{
    type InitChannelOptions = SovereignInitChannelOptions;
}
