use cgp_core::Async;

use crate::chain::traits::types::ibc::HasIbcChainTypes;

pub trait HasUpdateClientPayload<Counterparty>: HasIbcChainTypes<Counterparty> {
    type UpdateClientPayload: Async;
}
