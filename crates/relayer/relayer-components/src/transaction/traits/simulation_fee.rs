use cgp_core::prelude::*;

use crate::transaction::traits::types::fee::HasFeeType;

#[derive_component(FeeForSimulationGetterComponent, FeeForSimulationGetter<Chain>)]
pub trait HasFeeForSimulation: HasFeeType {
    fn fee_for_simulation(&self) -> &Self::Fee;
}
