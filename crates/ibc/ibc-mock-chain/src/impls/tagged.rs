use core::marker::PhantomData;

use cgp::core::types::{ProvideType, TypeComponent};
use hermes_prelude::*;

use crate::contexts::chain::MockChain;
use crate::types::tagged::Tagged;

pub struct UseTaggedType<Provider>(pub PhantomData<Provider>);

#[cgp_provider(TypeComponent)]
impl<A, B, Tag, Provider> ProvideType<MockChain<A, B>, Tag> for UseTaggedType<Provider>
where
    A: Async,
    B: Async,
    Provider: ProvideType<MockChain<A, B>, Tag>,
{
    type Type = Tagged<A, B, Provider::Type>;
}
