use cgp::prelude::*;
use hermes_protobuf_encoding_components::impls::encode_mut::message::EncodeProstMessage;
use hermes_protobuf_encoding_components::types::strategy::ViaProtobuf;

pub struct CosmosEncodeMutComponents;

delegate_components! {
    CosmosEncodeMutComponents {
        (ViaProtobuf, String): EncodeProstMessage,
    }
}
