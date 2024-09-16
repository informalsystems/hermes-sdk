use cgp::prelude::*;
use hermes_cosmos_encoding_components::components::CosmosEncodingComponents;
use hermes_encoding_components::impls::delegate::DelegateEncoding;
pub use hermes_encoding_components::traits::convert::ConverterComponent;
pub use hermes_encoding_components::traits::decode::DecoderComponent;
pub use hermes_encoding_components::traits::decode_mut::MutDecoderComponent;
pub use hermes_encoding_components::traits::encode::EncoderComponent;
pub use hermes_encoding_components::traits::encode_mut::MutEncoderComponent;
pub use hermes_encoding_components::traits::schema::SchemaGetterComponent;
pub use hermes_encoding_components::traits::types::decode_buffer::DecodeBufferTypeComponent;
pub use hermes_encoding_components::traits::types::encode_buffer::EncodeBufferTypeComponent;
pub use hermes_encoding_components::traits::types::encoded::EncodedTypeComponent;
pub use hermes_encoding_components::traits::types::schema::SchemaTypeComponent;
pub use hermes_protobuf_encoding_components::traits::length::EncodedLengthGetterComponent;

use crate::encoding::convert::CosmosConverterComponents;
use crate::encoding::encode::CosmosEncoderComponents;
use crate::encoding::type_url::CosmosTypeUrlSchemas;

define_components! {
    CosmosClientEncodingComponents {
        [
            EncodedTypeComponent,
            EncodeBufferTypeComponent,
            DecodeBufferTypeComponent,
            SchemaTypeComponent,
        ]:
            CosmosEncodingComponents,
        ConverterComponent:
            DelegateEncoding<CosmosConverterComponents>,
        [
            EncoderComponent,
            DecoderComponent,
        ]:
            DelegateEncoding<CosmosEncoderComponents>,
        [
            MutEncoderComponent,
            MutDecoderComponent,
            EncodedLengthGetterComponent,
        ]:
            CosmosEncodingComponents,
        SchemaGetterComponent:
            DelegateEncoding<CosmosTypeUrlSchemas>,
    }
}
