use ibc::core::host::types::identifiers::ClientId;

pub struct CosmosCreateClientEvent {
    pub client_id: ClientId,
}
