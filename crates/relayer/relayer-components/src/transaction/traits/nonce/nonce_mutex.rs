use cgp::prelude::*;
use hermes_runtime_components::traits::mutex::{HasMutex, MutexGuardOf, MutexOf};
use hermes_runtime_components::traits::runtime::HasRuntime;

use crate::transaction::traits::nonce::nonce_guard::HasNonceGuard;
use crate::transaction::traits::types::signer::HasSignerType;

/**
   A naive nonce allocator that simply query the current nonce from the context
   and then pass it to the continuation.

   To ensure that the nonce works safely with parallel transaction submissions,
   the allocator requires the context to provide a mutex, which is acquired across
   the time when the nonce is being allocated and used. Because of this, the naive
   allocator only allows one transaction to be submitted at a time.
*/
#[cgp_component {
  name: MutexForNonceAllocationComponent,
  provider: ProvideMutexForNonceAllocation,
  context: Chain,
}]
pub trait HasMutexForNonceAllocation:
    HasRuntime<Runtime: HasMutex> + HasNonceGuard + HasSignerType
{
    fn mutex_for_nonce_allocation<'a>(
        &'a self,
        signer: &Self::Signer,
    ) -> &'a MutexOf<Self::Runtime, ()>;

    fn mutex_to_nonce_guard<'a>(
        mutex_guard: MutexGuardOf<'a, Self::Runtime, ()>,
        nonce: Self::Nonce,
    ) -> Self::NonceGuard<'a>;
}
