use cgp::prelude::*;

#[cgp_type]
pub trait HasAnyCounterpartyType {
    type AnyCounterparty: Async;
}
