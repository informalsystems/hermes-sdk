use cgp_core::prelude::*;

pub trait HasGenesisConfigType: Async {
    type GenesisConfig: Async;
}
