use cgp_core::prelude::*;
use hermes_encoding_components::impls::convert_and_encode::ConvertAndEncode;
use hermes_encoding_components::types::via::Via;
use hermes_protobuf_encoding_components::impls::protobuf::EncodeAsProtobuf;
use hermes_protobuf_encoding_components::impls::via_any::EncodeViaAny;
use hermes_protobuf_encoding_components::types::Any;

use crate::types::client_state::{ProtoSolomachineClientState, SolomachineClientState};

pub struct SolomachineEncoderComponents;

delegate_components! {
    SolomachineEncoderComponents {
        Via<Any, SolomachineClientState>: EncodeViaAny,
        SolomachineClientState: ConvertAndEncode<ProtoSolomachineClientState>,
        ProtoSolomachineClientState: EncodeAsProtobuf,
    }
}
