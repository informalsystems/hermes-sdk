use cgp::prelude::*;
use hermes_cosmos_encoding_components::impls::height::EncodeHeight;
use hermes_protobuf_encoding_components::impls::encode_mut::any::EncodeAny;
use hermes_protobuf_encoding_components::impls::encode_mut::message::EncodeProstMessage;
use hermes_protobuf_encoding_components::types::strategy::ViaProtobuf;
use ibc::core::client::types::Height;
use prost_types::Any;

pub struct CosmosEncodeMutComponents;

delegate_components! {
    CosmosEncodeMutComponents {
        (ViaProtobuf, String): EncodeProstMessage,
        (ViaProtobuf, Height): EncodeHeight,
        (ViaProtobuf, Any): EncodeAny,
    }
}
