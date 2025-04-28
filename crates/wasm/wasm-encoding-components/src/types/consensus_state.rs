use hermes_prelude::*;

#[derive(Debug, HasField)]
pub struct WasmConsensusState {
    pub data: Vec<u8>,
}
