use cgp_core::prelude::*;
use hermes_cosmos_chain_components::encoding::components::CosmosEncodingComponents;
use hermes_cosmos_chain_components::types::tendermint::{
    TendermintClientState, TendermintConsensusState,
};
use hermes_protobuf_encoding_components::impl_type_url;
use hermes_sovereign_rollup_components::types::client_state::SovereignClientState;
use hermes_sovereign_rollup_components::types::consensus_state::SovereignConsensusState;
use hermes_wasm_client_components::impls::encoding::components::WasmEncodingComponents;
use hermes_wasm_client_components::types::client_state::WasmClientState;
use hermes_wasm_client_components::types::consensus_state::WasmConsensusState;

pub struct SovereignTypeUrlSchemas;

delegate_components! {
    SovereignTypeUrlSchemas {
        [
            TendermintClientState,
            TendermintConsensusState,
        ]:
            CosmosEncodingComponents,

        [
            WasmClientState,
            WasmConsensusState,
        ]:
            WasmEncodingComponents,

        SovereignClientState:
            SovereignClientStateUrl,
        SovereignConsensusState:
            SovereignConsensusStateUrl,
    }
}

impl_type_url!(
    SovereignClientStateUrl,
    "/ibc.lightclients.sovereign.tendermint.v1.ClientState",
);

impl_type_url!(
    SovereignConsensusStateUrl,
    "/ibc.lightclients.sovereign.tendermint.v1.ConsensusState",
);
