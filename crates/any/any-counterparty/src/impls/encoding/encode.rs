use cgp_core::prelude::*;
use hermes_cosmos_chain_components::encoding::components::CosmosEncodingComponents;
use hermes_protobuf_encoding_components::types::{Any, Protobuf};
use ibc_proto::ibc::lightclients::tendermint::v1::ClientState as ProtoTendermintClientState;
use ibc_relayer_types::clients::ics07_tendermint::client_state::ClientState as TendermintClientState;

use crate::impls::encoding::client_state::EncodeAnyClientState;
use crate::types::client_state::AnyClientState;

pub struct AnyClientEncoderComponents;

delegate_components! {
    AnyClientEncoderComponents {
        [
            (Any, TendermintClientState),
            (Protobuf, TendermintClientState),
            (Protobuf, Any),
            (Protobuf, ProtoTendermintClientState),
        ]:
            CosmosEncodingComponents,
        (Protobuf, AnyClientState): EncodeAnyClientState,
    }
}
