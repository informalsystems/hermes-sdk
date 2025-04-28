use hermes_core::relayer_components::chain::traits::{
    CreateClientMessageOptionsTypeComponent, ProvideCreateClientMessageOptionsType,
};
use hermes_prelude::*;

#[derive(Clone)]
pub struct CreateWasmTendermintMessageOptions {
    pub code_hash: Vec<u8>,
}

pub struct ProvidCreateWasmTendermintMessageOptionsType;

#[cgp_provider(CreateClientMessageOptionsTypeComponent)]
impl<Chain, Counterparty> ProvideCreateClientMessageOptionsType<Chain, Counterparty>
    for ProvidCreateWasmTendermintMessageOptionsType
where
    Chain: Async,
{
    type CreateClientMessageOptions = CreateWasmTendermintMessageOptions;
}
