use hermes_relayer_components::chain::traits::types::channel::ProvideInitChannelOptionsType;

use crate::types::channel::CosmosInitChannelOptions;

pub struct ProvideCosmosInitChannelOptionsType;

impl<Chain, Counterparty> ProvideInitChannelOptionsType<Chain, Counterparty>
    for ProvideCosmosInitChannelOptionsType
{
    type InitChannelOptions = CosmosInitChannelOptions;
}
