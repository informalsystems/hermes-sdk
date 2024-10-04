use core::marker::PhantomData;

use alloc::string::String;
use cgp::core::error::ErrorTypeComponent;
use cgp::core::types::traits::ProvideType;
use cgp::prelude::*;

define_components! {
    MockChainTypes {
        ErrorTypeComponent: String,
    }
}

pub struct UseDelegatedType<Components>(pub PhantomData<Components>);

impl<Context, Tag, Components, Type> ProvideType<Context, Tag> for UseDelegatedType<Components>
where
    Components: DelegateComponent<Tag, Delegate = Type>,
{
    type Type = Type;
}
