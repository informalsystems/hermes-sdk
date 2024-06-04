use cgp_core::Async;
use hermes_cosmos_chain_components::types::tendermint::{
    TendermintClientState, TendermintConsensusState,
};
use hermes_relayer_components::chain::traits::types::create_client::{
    ProvideCreateClientOptionsType, ProvideCreateClientPayloadType,
};
use ibc_relayer::chain::client::ClientSettings;

#[derive(Clone)]
pub struct CreateWasmTendermintClientOptions {
    pub client_settings: ClientSettings,
    pub code_hash: Vec<u8>,
}

pub struct CreateWasmTendermintClientPayload {
    pub client_state: TendermintClientState,
    pub consensus_state: TendermintConsensusState,
    pub code_hash: Vec<u8>,
}

pub struct ProvideWasmTendermintClientTypes;

impl<Chain, Counterparty> ProvideCreateClientOptionsType<Chain, Counterparty>
    for ProvideWasmTendermintClientTypes
where
    Chain: Async,
{
    type CreateClientOptions = CreateWasmTendermintClientOptions;
}

impl<Chain, Counterparty> ProvideCreateClientPayloadType<Chain, Counterparty>
    for ProvideWasmTendermintClientTypes
where
    Chain: Async,
{
    type CreateClientPayload = CreateWasmTendermintClientPayload;
}
