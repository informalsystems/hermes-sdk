use cgp_core::prelude::*;

#[derive_component(EventIdTypeComponent, ProvideEventIdType<Rollup>)]
pub trait HasEventIdType: Async {
    type EventId: Async;
}
