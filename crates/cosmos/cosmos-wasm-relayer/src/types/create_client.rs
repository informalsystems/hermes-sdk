use cgp::prelude::*;
use hermes_relayer_components::chain::traits::types::create_client::{
    CreateClientMessageOptionsTypeComponent, ProvideCreateClientMessageOptionsType,
};

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
