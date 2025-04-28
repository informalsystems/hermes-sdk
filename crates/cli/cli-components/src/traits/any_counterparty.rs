use hermes_prelude::*;

#[cgp_type]
pub trait HasAnyCounterpartyType {
    type AnyCounterparty: Async;
}
