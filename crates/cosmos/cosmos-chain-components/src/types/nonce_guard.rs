use core::ops::Deref;

use futures::lock::MutexGuard;

use crate::types::transaction::account::Account;

pub struct NonceGuard<'a> {
    pub mutex_guard: MutexGuard<'a, ()>,
    pub account: Account,
}

impl Deref for NonceGuard<'_> {
    type Target = Account;

    fn deref(&self) -> &Account {
        &self.account
    }
}
