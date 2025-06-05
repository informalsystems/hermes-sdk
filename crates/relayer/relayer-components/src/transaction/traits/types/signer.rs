use core::fmt::Debug;

use hermes_prelude::*;

#[cgp_type]
pub trait HasSignerType: Async {
    type Signer: Async + Debug;
}
