use cgp::prelude::*;

#[cgp_component {
  name: FeeTypeComponent,
  provider: ProvideFeeType,
  context: Chain,
}]
pub trait HasFeeType: Async {
    type Fee: Async;
}
