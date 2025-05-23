use hermes_core::relayer_components::chain::traits::{
    InitConnectionOptionsTypeComponent, ProvideInitConnectionOptionsType,
};
use hermes_prelude::*;

use crate::types::CosmosInitConnectionOptions;

pub struct ProvideCosmosInitConnectionOptionsType;

#[cgp_provider(InitConnectionOptionsTypeComponent)]
impl<Chain, Counterparty> ProvideInitConnectionOptionsType<Chain, Counterparty>
    for ProvideCosmosInitConnectionOptionsType
where
    Chain: Async,
{
    type InitConnectionOptions = CosmosInitConnectionOptions;
}
