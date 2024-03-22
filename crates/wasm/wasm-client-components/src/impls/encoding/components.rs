use cgp_core::prelude::*;
use hermes_encoding_components::impls::delegate::DelegateEncoding;
use hermes_encoding_components::impls::encoded::ProvideEncodedBytes;
use hermes_encoding_components::impls::schema::ProvideStringSchema;
use hermes_encoding_components::traits::convert::ConverterComponent;
use hermes_encoding_components::traits::decoder::DecoderComponent;
use hermes_encoding_components::traits::encoded::EncodedTypeComponent;
use hermes_encoding_components::traits::encoder::EncoderComponent;
use hermes_encoding_components::traits::schema::{SchemaGetterComponent, SchemaTypeComponent};

use crate::impls::encoding::convert::WasmConverterComponents;
use crate::impls::encoding::encoder::WasmEncoderComponents;
use crate::impls::encoding::type_url::WasmTypeUrlSchemas;

pub struct WasmEncodingComponents;

delegate_components! {
    #[mark_component(IsWasmEncodingComponent)]
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
