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
