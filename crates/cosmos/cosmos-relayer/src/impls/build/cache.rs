use alloc::collections::BTreeMap;

use futures::lock::Mutex;
use hermes_error::types::Error;
use hermes_relayer_components::build::traits::cache::{HasChainCache, HasRelayCache};
use hermes_relayer_components::build::traits::target::chain::{ChainATarget, ChainBTarget};
use hermes_relayer_components::build::traits::target::relay::{RelayAToBTarget, RelayBToATarget};
use hermes_relayer_components_extra::build::traits::cache::HasBatchSenderCache;
use ibc_relayer_types::core::ics24_host::identifier::{ChainId, ClientId};

use crate::contexts::builder::CosmosBuilder;
use crate::contexts::chain::CosmosChain;
use crate::contexts::relay::CosmosRelay;
use crate::types::batch::CosmosBatchSender;

impl HasChainCache<ChainATarget> for CosmosBuilder {
    fn chain_cache(&self) -> &Mutex<BTreeMap<ChainId, CosmosChain>> {
        &self.chain_cache
    }
}

impl HasChainCache<ChainBTarget> for CosmosBuilder {
    fn chain_cache(&self) -> &Mutex<BTreeMap<ChainId, CosmosChain>> {
        &self.chain_cache
    }
}

impl HasRelayCache<RelayAToBTarget> for CosmosBuilder {
    fn relay_cache(&self) -> &Mutex<BTreeMap<(ChainId, ChainId, ClientId, ClientId), CosmosRelay>> {
        &self.relay_cache
    }
}

impl HasRelayCache<RelayBToATarget> for CosmosBuilder {
    fn relay_cache(&self) -> &Mutex<BTreeMap<(ChainId, ChainId, ClientId, ClientId), CosmosRelay>> {
        &self.relay_cache
    }
}

impl HasBatchSenderCache<ChainATarget, Error> for CosmosBuilder {
    fn batch_sender_cache(
        &self,
        _target: ChainATarget,
    ) -> &Mutex<BTreeMap<(ChainId, ChainId, ClientId, ClientId), CosmosBatchSender>> {
        &self.batch_senders
    }
}

impl HasBatchSenderCache<ChainBTarget, Error> for CosmosBuilder {
    fn batch_sender_cache(
        &self,
        _target: ChainBTarget,
    ) -> &Mutex<BTreeMap<(ChainId, ChainId, ClientId, ClientId), CosmosBatchSender>> {
        &self.batch_senders
    }
}
