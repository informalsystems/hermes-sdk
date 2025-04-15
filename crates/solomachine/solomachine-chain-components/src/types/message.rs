use hermes_cosmos_chain_components::types::{CosmosCreateClientPayload, CosmosUpdateClientPayload};

#[derive(Debug)]
pub enum SolomachineMessage {
    CosmosCreateClient(Box<CosmosCreateClientPayload>),
    CosmosUpdateClient(Box<CosmosUpdateClientPayload>),
    CosmosConnectionOpenInit { commitment_prefix: Vec<u8> },
}
