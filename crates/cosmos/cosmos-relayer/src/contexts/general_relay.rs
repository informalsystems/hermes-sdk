use alloc::sync::Arc;
use core::marker::PhantomData;

use cgp::prelude::*;

pub struct RelayContext<Components> {
    pub fields: Arc<dyn HasRelayContextFields>,
    pub phantom: PhantomData<Components>,
}

pub trait HasRelayContextFields: Async {
    fn relay_context_fields(&self) -> &RelayContextFields;
}

pub struct RelayContextFields {}

pub trait RelayContextComponents {
    type Runtime;

    type ChainA;

    type ChainB;
}
