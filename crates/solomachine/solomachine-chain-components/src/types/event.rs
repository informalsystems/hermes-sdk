use hermes_cosmos_chain_components::types::tendermint::TendermintClientState;
use ibc::core::host::types::identifiers::{ClientId, ConnectionId};

#[derive(Clone)]
pub enum SolomachineEvent {
    ConnectionInit(SolomachineConnectionInitEvent),
    CreateClient(SolomachineCreateClientEvent),
}

#[derive(Clone)]
pub struct SolomachineCreateClientEvent {
    pub client_id: ClientId,
    pub client_state: TendermintClientState,
}

#[derive(Clone)]
pub struct SolomachineConnectionInitEvent {
    pub connection_id: ConnectionId,
}
