use cgp_core::prelude::*;
use hermes_protobuf_components::impl_type_url;

use crate::types::client_state::SolomachineClientState;

pub struct SolomachineTypeUrlSchemas;

delegate_components! {
    SolomachineTypeUrlSchemas {
        SolomachineClientState: SolomachineClientStateUrl,
    }
}

impl_type_url!(
    SolomachineClientStateUrl,
    "/ibc.lightclients.solomachine.v3.ClientState"
);
