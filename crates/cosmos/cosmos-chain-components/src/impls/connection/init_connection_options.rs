use cgp::prelude::*;
use hermes_relayer_components::chain::traits::types::connection::{
    InitConnectionOptionsTypeComponent, ProvideInitConnectionOptionsType,
};

use crate::types::connection::CosmosInitConnectionOptions;

pub struct ProvideCosmosInitConnectionOptionsType;

#[cgp_provider(InitConnectionOptionsTypeComponent)]
impl<Chain, Counterparty> ProvideInitConnectionOptionsType<Chain, Counterparty>
    for ProvideCosmosInitConnectionOptionsType
where
    Chain: Async,
{
    type InitConnectionOptions = CosmosInitConnectionOptions;
}
