use hermes_prelude::*;

use crate::transaction::traits::HasFeeType;

#[cgp_component {
  provider: FeeForSimulationGetter,
  context: Chain,
}]
pub trait HasFeeForSimulation: HasFeeType {
    fn fee_for_simulation(&self) -> &Self::Fee;
}
