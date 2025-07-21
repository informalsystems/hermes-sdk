use core::fmt::Debug;

use hermes_prelude::*;

#[cgp_type]
pub trait HasSetupUpgradeClientTestResultType {
    type SetupUpgradeClientTestResult: Async + Debug;
}
