use cgp::prelude::Async;
use hermes_relayer_components::chain::traits::types::connection::ProvideInitConnectionOptionsType;

use crate::types::connection::CosmosInitConnectionOptions;

pub struct ProvideCosmosInitConnectionOptionsType;

impl<Chain, Counterparty> ProvideInitConnectionOptionsType<Chain, Counterparty>
    for ProvideCosmosInitConnectionOptionsType
where
    Chain: Async,
{
    type InitConnectionOptions = CosmosInitConnectionOptions;
}
