use hermes_prelude::*;
use hermes_protobuf_encoding_components::impl_type_url;

use crate::types::client_state::{SolomachineClientState, SOLOMACHINE_CLIENT_STATE_TYPE_URL};
use crate::types::consensus_state::{
    SolomachineConsensusState, SOLOMACHINE_CONSENSUS_STATE_TYPE_URL,
};

pub struct SolomachineTypeUrlSchemas;

impl_type_url!(
    SolomachineTypeUrlSchemas,
    SolomachineClientState,
    SOLOMACHINE_CLIENT_STATE_TYPE_URL,
);

impl_type_url!(
    SolomachineTypeUrlSchemas,
    SolomachineConsensusState,
    SOLOMACHINE_CONSENSUS_STATE_TYPE_URL,
);
