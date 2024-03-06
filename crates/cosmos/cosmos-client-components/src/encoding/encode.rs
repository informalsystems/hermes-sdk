use cgp_core::prelude::*;
use hermes_protobuf_components::impls::protobuf::EncodeAsProtobuf;
use hermes_protobuf_components::impls::via_any::EncodeViaAny;
use hermes_relayer_components::encode::impls::convert_and_encode::ConvertAndEncode;
use hermes_relayer_components::encode::types::via::Via;
use ibc_proto::ibc::lightclients::tendermint::v1::ClientState as ProtoTendermintClientState;
use ibc_relayer_types::clients::ics07_tendermint::client_state::ClientState as TendermintClientState;
use prost_types::Any;

pub struct CosmosEncoderComponents;

delegate_components! {
    CosmosEncoderComponents {
        Via<Any, TendermintClientState>: EncodeViaAny,

        TendermintClientState: ConvertAndEncode<ProtoTendermintClientState>,

        Any: EncodeAsProtobuf,
        ProtoTendermintClientState: EncodeAsProtobuf,
    }
}
