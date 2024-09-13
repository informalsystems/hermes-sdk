use cgp::prelude::*;
use hermes_encoding_components::impls::encode_mut::pair::EncoderPair;
use hermes_encoding_components::impls::from_context::EncodeFromContext;
use hermes_protobuf_encoding_components::impls::encode_mut::message::EncodeProstMessage;
use hermes_protobuf_encoding_components::types::strategy::ViaProtobuf;
use ibc::core::client::types::Height;

use crate::impls::encoding::height::EncodeHeight;

pub struct CosmosEncodeMutComponents;

delegate_components! {
    CosmosEncodeMutComponents {
        <A, B> (ViaProtobuf, (A, B)): EncoderPair<EncodeFromContext, EncodeFromContext>,
        (ViaProtobuf, String): EncodeProstMessage,
        (ViaProtobuf, Height): EncodeHeight,
    }
}
