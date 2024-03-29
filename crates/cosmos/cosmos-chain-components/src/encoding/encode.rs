use cgp_core::prelude::*;
use hermes_encoding_components::impls::convert_and_encode::ConvertAndEncode;
use hermes_encoding_components::impls::via_identity::{EncodeViaIdentity, Identity};
use hermes_encoding_components::types::via::Via;
use hermes_protobuf_encoding_components::impls::protobuf::EncodeAsProtobuf;
use hermes_protobuf_encoding_components::impls::via_any::EncodeViaAny;
use ibc_proto::ibc::lightclients::tendermint::v1::ClientState as ProtoTendermintClientState;
use ibc_relayer_types::clients::ics07_tendermint::client_state::ClientState as TendermintClientState;
use prost_types::Any;

pub struct CosmosEncoderComponents;

delegate_components! {
    CosmosEncoderComponents {
        Via<Any, TendermintClientState>: EncodeViaAny,
        Via<Identity, TendermintClientState>: EncodeViaIdentity,

        TendermintClientState: ConvertAndEncode<ProtoTendermintClientState>,

        Any: EncodeAsProtobuf,
        ProtoTendermintClientState: EncodeAsProtobuf,
    }
}
