use cgp_core::prelude::*;
use hermes_relayer_components::encode::impls::delegate::DelegateEncoding;
use hermes_relayer_components::encode::impls::encoded::ProvideEncodedBytes;
use hermes_relayer_components::encode::impls::schema::ProvideStringSchema;
use hermes_relayer_components::encode::traits::convert::ConverterComponent;
use hermes_relayer_components::encode::traits::decoder::DecoderComponent;
use hermes_relayer_components::encode::traits::encoded::EncodedTypeComponent;
use hermes_relayer_components::encode::traits::encoder::EncoderComponent;
use hermes_relayer_components::encode::traits::schema::{
    SchemaGetterComponent, SchemaTypeComponent,
};

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
