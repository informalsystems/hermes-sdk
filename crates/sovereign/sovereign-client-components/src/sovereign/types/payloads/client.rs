use hermes_cosmos_client_components::types::payloads::client::{
    CosmosCreateClientPayload, CosmosUpdateClientPayload,
};
use ibc_relayer_types::core::ics02_client::height::Height;

pub struct SovereignCreateClientPayload {
    pub celestia_payload: CosmosCreateClientPayload,
    // TODO: Add rollup payloads
    pub code_hash: Vec<u8>,
    pub latest_height: Height,
}

pub struct SovereignUpdateClientPayload {
    pub celestia_payload: CosmosUpdateClientPayload,
    // TODO: Add rollup payloads
}
