use alloc::sync::Arc;
use core::marker::PhantomData;

use cgp::prelude::*;
use futures::lock::Mutex;

use crate::transaction::traits::{
    HasSignerType, NonceAllocationMutexGetter, NonceAllocationMutexGetterComponent,
};

/**
   A provider that returns the same global mutex from a chain regardless of the signer.

   This would mean that the transactions would be queued by the same lock, even when
   different signers are used. This is less efficient than using per-signer mutex,
   but this is easier to use, especially for chains that only have one default signer
   that is used together with `SendMessagesWithDefaultSigner`.
*/
#[cgp_new_provider(NonceAllocationMutexGetterComponent)]
impl<Chain, Tag> NonceAllocationMutexGetter<Chain> for GetGlobalNonceMutex<Tag>
where
    Chain: HasSignerType + HasField<Tag, Value = Arc<Mutex<()>>>,
{
    fn mutex_for_nonce_allocation<'a>(chain: &'a Chain, _signer: &Chain::Signer) -> &'a Mutex<()> {
        chain.get_field(PhantomData)
    }
}
