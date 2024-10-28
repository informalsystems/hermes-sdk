use ibc_relayer_types::clients::ics07_tendermint::client_state::ClientState;
use ibc_relayer_types::core::ics24_host::identifier::{ClientId, ConnectionId};

#[derive(Clone)]
pub enum SolomachineEvent {
    ConnectionInit(SolomachineConnectionInitEvent),
    CreateClient(SolomachineCreateClientEvent),
}

#[derive(Clone)]
pub struct SolomachineCreateClientEvent {
    pub client_id: ClientId,
    pub client_state: ClientState,
}

#[derive(Clone)]
pub struct SolomachineConnectionInitEvent {
    pub connection_id: ConnectionId,
}
