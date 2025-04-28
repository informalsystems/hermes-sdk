use futures::lock::Mutex;
use hermes_prelude::*;

use crate::transaction::traits::HasSignerType;

/**
   A naive nonce allocator that simply query the current nonce from the context
   and then pass it to the continuation.

   To ensure that the nonce works safely with parallel transaction submissions,
   the allocator requires the context to provide a mutex, which is acquired across
   the time when the nonce is being allocated and used. Because of this, the naive
   allocator only allows one transaction to be submitted at a time.
*/
#[cgp_component {
    name: NonceAllocationMutexGetterComponent,
    provider: NonceAllocationMutexGetter,
    context: Chain,
}]
pub trait HasMutexForNonceAllocation: HasSignerType {
    fn mutex_for_nonce_allocation<'a>(&'a self, signer: &Self::Signer) -> &'a Mutex<()>;
}
