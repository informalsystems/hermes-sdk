use core::marker::PhantomData;

use alloc::string::String;
use cgp::core::error::ErrorTypeComponent;
use cgp::core::types::traits::ProvideType;
use cgp::prelude::*;
use hermes_chain_type_components::traits::types::ibc::channel_id::ChannelIdTypeComponent;
use hermes_ibc_components::traits::types::app_id::AppIdTypeComponent;

use crate::types::app_id::MockAppId;
use crate::types::channel_id::MockChannelId;

define_components! {
    MockChainTypes {
        ErrorTypeComponent: String,
        AppIdTypeComponent: MockAppId,
        ChannelIdTypeComponent: MockChannelId,
    }
}

pub struct UseDelegatedType<Components>(pub PhantomData<Components>);

impl<Context, Tag, Components, Type> ProvideType<Context, Tag> for UseDelegatedType<Components>
where
    Components: DelegateComponent<Tag, Delegate = Type>,
{
    type Type = Type;
}
