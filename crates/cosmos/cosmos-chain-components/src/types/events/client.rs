use ibc::core::host::types::identifiers::ClientId;
use prost_types::Any;

pub struct CosmosCreateClientEvent {
    pub client_id: ClientId,
}

#[derive(Debug, Clone)]
pub struct CosmosUpdateClientEvent {
    pub client_id: ClientId,
    pub header: Any,
}
