use cgp::prelude::*;
use hermes_protobuf_encoding_components::impls::encode_mut::message::EncodeProstMessage;
use hermes_protobuf_encoding_components::types::strategy::ViaProtobuf;
use ibc::core::client::types::Height;

use crate::impls::encoding::height::EncodeHeight;

pub struct CosmosEncodeMutComponents;

delegate_components! {
    CosmosEncodeMutComponents {
        (ViaProtobuf, String): EncodeProstMessage,
        (ViaProtobuf, Height): EncodeHeight,
    }
}
