use hermes_prelude::*;

#[cgp_type]
pub trait HasFeeType: Async {
    type Fee: Async;
}
