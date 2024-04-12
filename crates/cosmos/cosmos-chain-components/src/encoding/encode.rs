use cgp_core::prelude::*;
use hermes_encoding_components::impls::convert_and_encode::ConvertAndEncode;
use hermes_encoding_components::impls::via_identity::{EncodeViaIdentity, Identity};
use hermes_encoding_components::types::via::Via;
use hermes_protobuf_encoding_components::impls::protobuf::EncodeAsProtobuf;
use hermes_protobuf_encoding_components::impls::via_any::EncodeViaAny;
use prost_types::Any;

use crate::types::tendermint::{
    ProtoTendermintClientState, ProtoTendermintConsensusState, TendermintClientState,
    TendermintConsensusState,
};

pub struct CosmosEncoderComponents;

delegate_components! {
    CosmosEncoderComponents {
        Via<Any, TendermintClientState>: EncodeViaAny,
        Via<Identity, TendermintClientState>: EncodeViaIdentity,

        TendermintClientState: ConvertAndEncode<ProtoTendermintClientState>,
        ProtoTendermintClientState: EncodeAsProtobuf,

        Via<Any, TendermintConsensusState>: EncodeViaAny,
        Via<Identity, TendermintConsensusState>: EncodeViaIdentity,

        TendermintConsensusState: ConvertAndEncode<ProtoTendermintConsensusState>,
        ProtoTendermintConsensusState: EncodeAsProtobuf,

        Any: EncodeAsProtobuf,
    }
}
