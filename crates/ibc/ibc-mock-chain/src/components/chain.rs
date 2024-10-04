use cgp::core::component::WithContext;
use cgp::core::error::{ErrorRaiserComponent, ErrorTypeComponent};
use cgp::core::types::traits::TypeComponent;
use cgp::prelude::*;

use crate::impls::error::RaiseDebugString;
use crate::impls::types::{MockChainTypes, UseDelegatedType};

define_components! {
    MockChainComponents {
        TypeComponent:
            UseDelegatedType<MockChainTypes>,
        [
            ErrorTypeComponent,
        ]:
            WithContext,
        ErrorRaiserComponent:
            RaiseDebugString,
    }
}
