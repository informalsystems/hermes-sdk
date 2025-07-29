use alloc::sync::Arc;
use alloc::vec::Vec;
use core::marker::PhantomData;

use futures::lock::Mutex;
use hermes_prelude::*;

use crate::transaction::traits::{HasSignerType, SignerMutexGetter, SignerMutexGetterComponent};

#[cgp_new_provider(SignerMutexGetterComponent)]
impl<Chain, Tag, AdditionalTag> SignerMutexGetter<Chain>
    for GetGlobalSignerMutex<Tag, AdditionalTag>
where
    Chain: HasSignerType
        + HasField<Tag, Value = Arc<Mutex<usize>>>
        + HasField<AdditionalTag, Value = Vec<Chain::Signer>>,
{
    fn mutex_for_signer<'a>(chain: &'a Chain) -> (&'a Mutex<usize>, usize) {
        let additional_signers_len = chain.get_field(PhantomData::<AdditionalTag>).len();
        (chain.get_field(PhantomData::<Tag>), additional_signers_len)
    }
}
