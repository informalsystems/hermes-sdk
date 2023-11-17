use std::path::Path;

use cgp_core::prelude::*;

pub trait HasChainCommandPath: Async {
    fn chain_command_path(&self) -> &Path;
}
