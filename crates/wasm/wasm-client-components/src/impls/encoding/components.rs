use cgp_core::prelude::*;
use hermes_encoding_components::impls::delegate::DelegateEncoding;
use hermes_encoding_components::impls::encoded::ProvideEncodedBytes;
use hermes_encoding_components::impls::schema::ProvideStringSchema;
pub use hermes_encoding_components::traits::convert::ConverterComponent;
pub use hermes_encoding_components::traits::decoder::DecoderComponent;
pub use hermes_encoding_components::traits::encoder::EncoderComponent;
pub use hermes_encoding_components::traits::schema::SchemaGetterComponent;
pub use hermes_encoding_components::traits::types::encoded::EncodedTypeComponent;
pub use hermes_encoding_components::traits::types::schema::SchemaTypeComponent;

use crate::impls::encoding::convert::WasmConverterComponents;
use crate::impls::encoding::encoder::WasmEncoderComponents;
use crate::impls::encoding::type_url::WasmTypeUrlSchemas;

define_components! {
    WasmEncodingComponents {
        EncodedTypeComponent:
            ProvideEncodedBytes,
        SchemaTypeComponent:
            ProvideStringSchema,
        ConverterComponent:
            DelegateEncoding<WasmConverterComponents>,
        [
            EncoderComponent,
            DecoderComponent,
        ]:
            DelegateEncoding<WasmEncoderComponents>,
        SchemaGetterComponent:
            DelegateEncoding<WasmTypeUrlSchemas>,
    }
}
