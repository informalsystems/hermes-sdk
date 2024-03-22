use cgp_core::prelude::*;
use hermes_encoding_components::impls::delegate::DelegateEncoding;
use hermes_encoding_components::impls::encoded::ProvideEncodedBytes;
use hermes_encoding_components::impls::schema::ProvideStringSchema;
use hermes_encoding_components::traits::convert::ConverterComponent;
use hermes_encoding_components::traits::decoder::DecoderComponent;
use hermes_encoding_components::traits::encoded::EncodedTypeComponent;
use hermes_encoding_components::traits::encoder::EncoderComponent;
use hermes_encoding_components::traits::schema::{SchemaGetterComponent, SchemaTypeComponent};

use crate::encoding::impls::convert::SovereignConverterComponents;
use crate::encoding::impls::encoder::SovereignEncoderComponents;
use crate::encoding::impls::type_url::SovereignTypeUrlSchemas;

pub struct SovereignEncodingComponents;

delegate_components! {
    #[mark_component(IsSovereignEncodingComponent)]
    SovereignEncodingComponents {
        EncodedTypeComponent:
            ProvideEncodedBytes,
        SchemaTypeComponent:
            ProvideStringSchema,
        ConverterComponent:
            DelegateEncoding<SovereignConverterComponents>,
        [
            EncoderComponent,
            DecoderComponent,
        ]:
            DelegateEncoding<SovereignEncoderComponents>,
        SchemaGetterComponent:
            DelegateEncoding<SovereignTypeUrlSchemas>,
    }
}
