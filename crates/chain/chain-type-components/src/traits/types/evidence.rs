use core::fmt::Debug;

use hermes_prelude::*;

#[cgp_type]
pub trait HasEvidenceType: Async {
    type Evidence: Debug + Async;
}
