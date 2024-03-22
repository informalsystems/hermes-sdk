use core::ops::Deref;

use futures::lock::MutexGuard;
use ibc_relayer::chain::cosmos::types::account::Account;

pub struct NonceGuard<'a> {
    pub mutex_guard: MutexGuard<'a, ()>,
    pub account: Account,
}

impl<'a> Deref for NonceGuard<'a> {
    type Target = Account;

    fn deref(&self) -> &Account {
        &self.account
    }
}
