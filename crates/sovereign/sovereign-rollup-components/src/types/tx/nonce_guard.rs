use core::ops::Deref;

use futures::lock::MutexGuard;

pub struct SovereignNonceGuard<'a> {
    pub mutex_guard: MutexGuard<'a, ()>,
    pub nonce: u64,
}

impl<'a> Deref for SovereignNonceGuard<'a> {
    type Target = u64;

    fn deref(&self) -> &u64 {
        &self.nonce
    }
}
