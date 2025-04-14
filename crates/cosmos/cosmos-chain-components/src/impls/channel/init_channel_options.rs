use cgp::prelude::*;
use hermes_relayer_components::chain::traits::{
    InitChannelOptionsTypeComponent, ProvideInitChannelOptionsType,
};

use crate::types::channel::CosmosInitChannelOptions;

pub struct ProvideCosmosInitChannelOptionsType;

#[cgp_provider(InitChannelOptionsTypeComponent)]
impl<Chain, Counterparty> ProvideInitChannelOptionsType<Chain, Counterparty>
    for ProvideCosmosInitChannelOptionsType
{
    type InitChannelOptions = CosmosInitChannelOptions;
}
