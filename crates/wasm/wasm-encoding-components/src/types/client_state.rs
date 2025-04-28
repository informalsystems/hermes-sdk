use hermes_prelude::*;
use ibc::core::client::types::Height;

#[derive(Clone, Debug, HasField, PartialEq, Eq)]
pub struct WasmClientState {
    pub data: Vec<u8>,
    pub checksum: Vec<u8>,
    pub latest_height: Height,
}
