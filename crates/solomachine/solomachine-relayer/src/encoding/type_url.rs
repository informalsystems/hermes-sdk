use cgp_core::prelude::*;
use hermes_protobuf_encoding_components::impl_type_url;

use crate::types::client_state::SolomachineClientState;
use crate::types::consensus_state::SolomachineConsensusState;

pub struct SolomachineTypeUrlSchemas;

delegate_components! {
    SolomachineTypeUrlSchemas {
        SolomachineClientState: SolomachineClientStateUrl,
        SolomachineConsensusState: SolomachineConsensusStateUrl,
    }
}

impl_type_url!(
    SolomachineClientStateUrl,
    "/ibc.lightclients.solomachine.v3.ClientState"
);

impl_type_url!(
    SolomachineConsensusStateUrl,
    "/ibc.lightclients.solomachine.v3.ConsensusState"
);
