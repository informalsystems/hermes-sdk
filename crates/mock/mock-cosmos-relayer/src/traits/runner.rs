use std::fmt::Debug;

use basecoin_store::context::ProvableStore;
use cgp_core::prelude::*;

/// Defines the interface for running a mock Cosmos chain.
#[async_trait]
pub trait BasecoinRunner {
    type Store: ProvableStore + Debug;

    async fn init(&self);

    async fn begin_block(&self);

    async fn commit(&self);
}
