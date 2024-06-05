use cgp_core::Async;
use hermes_relayer_components::chain::traits::types::create_client::ProvideCreateClientMessageOptionsType;

#[derive(Clone)]
pub struct CreateWasmTendermintMessageOptions {
    pub code_hash: Vec<u8>,
}

pub struct ProvidCreateWasmTendermintMessageOptionsType;

impl<Chain, Counterparty> ProvideCreateClientMessageOptionsType<Chain, Counterparty>
    for ProvidCreateWasmTendermintMessageOptionsType
where
    Chain: Async,
{
    type CreateClientMessageOptions = CreateWasmTendermintMessageOptions;
}
