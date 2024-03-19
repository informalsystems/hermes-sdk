use core::ops::Deref;

use crate::transaction::traits::types::HasNonceType;

pub trait HasNonceGuard: HasNonceType {
    type NonceGuard<'a>: Deref<Target = Self::Nonce> + Send + Sync;
}
