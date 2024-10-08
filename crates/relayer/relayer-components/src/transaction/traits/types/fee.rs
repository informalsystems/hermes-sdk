use cgp::prelude::*;

#[derive_component(FeeTypeComponent, ProvideFeeType<Chain>)]
pub trait HasFeeType: Async {
    type Fee: Async;
}
