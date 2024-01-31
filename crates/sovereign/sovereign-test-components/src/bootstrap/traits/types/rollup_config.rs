use cgp_core::prelude::*;

pub trait HasRollupConfigType: Async {
    type RollupConfig: Async;
}
