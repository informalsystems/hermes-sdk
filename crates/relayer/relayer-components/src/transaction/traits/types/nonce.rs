use cgp::prelude::*;

#[cgp_type]
pub trait HasNonceType: Async {
    type Nonce: Async;
}
