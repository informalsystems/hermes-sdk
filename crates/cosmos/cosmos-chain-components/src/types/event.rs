use std::sync::Arc;

pub use tendermint::abci::Event as AbciEvent;

pub type CosmosEvent = Arc<AbciEvent>;
