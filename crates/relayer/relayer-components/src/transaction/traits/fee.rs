use crate::transaction::traits::types::HasFeeType;

pub trait HasFeeForSimulation: HasFeeType {
    fn fee_for_simulation(&self) -> &Self::Fee;
}
