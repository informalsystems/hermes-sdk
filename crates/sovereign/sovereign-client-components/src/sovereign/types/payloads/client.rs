use hermes_cosmos_client_components::types::payloads::client::CosmosUpdateClientPayload;

pub struct SovereignCreateClientPayload {
    // pub celestia_payload: CosmosCreateClientPayload,
    // pub rollup_commitment_id: [u8; 32];
    // TODO: Add rollup payloads
}

pub struct SovereignUpdateClientPayload {
    pub celestia_payload: CosmosUpdateClientPayload,
    // TODO: Add rollup payloads
}
