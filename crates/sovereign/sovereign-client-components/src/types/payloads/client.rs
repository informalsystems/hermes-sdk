use hermes_cosmos_client_components::types::payloads::client::{
    CosmosCreateClientPayload, CosmosUpdateClientPayload,
};

pub struct SovereignCreateClientPayload {
    pub celestia_payload: CosmosCreateClientPayload,
    // TODO: Add rollup payloads
}

pub struct SovereignUpdateClientPayload {
    pub celestia_payload: CosmosUpdateClientPayload,
    // TODO: Add rollup payloads
}
