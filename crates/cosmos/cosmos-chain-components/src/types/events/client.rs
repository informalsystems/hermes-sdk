use ibc::core::host::types::identifiers::ClientId;
use ibc_client_tendermint::types::proto::v1::Header;

pub struct CosmosCreateClientEvent {
    pub client_id: ClientId,
}

#[derive(Debug, Clone)]
pub struct CosmosUpdateClientEvent {
    pub client_id: ClientId,
    pub header: Header,
}
