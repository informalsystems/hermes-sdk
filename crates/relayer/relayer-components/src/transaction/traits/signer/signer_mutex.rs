use futures::lock::Mutex;
use hermes_prelude::*;

#[cgp_component {
    name: SignerMutexGetterComponent,
    provider: SignerMutexGetter,
    context: Chain,
}]
pub trait HasMutexForSigner {
    fn mutex_for_signer<'a>(&'a self) -> (&'a Mutex<usize>, usize);
}
