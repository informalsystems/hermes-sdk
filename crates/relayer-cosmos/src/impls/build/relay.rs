use async_trait::async_trait;
use ibc_relayer::chain::handle::BaseChainHandle;
use ibc_relayer_all_in_one::one_for_all::types::chain::OfaChainWrapper;
use ibc_relayer_components::build::traits::target::relay::{RelayAToBTarget, RelayBToATarget};
use ibc_relayer_components_extra::build::traits::components::relay_with_batch_builder::RelayWithBatchBuilder;
use ibc_relayer_types::core::ics24_host::identifier::ClientId;

use crate::contexts::builder::CosmosBuilder;
use crate::contexts::chain::CosmosChain;
use crate::contexts::relay::CosmosRelay;
use crate::impls::build::components::CosmosBuildComponents;
use crate::types::batch::CosmosBatchSender;
use crate::types::error::Error;

#[async_trait]
impl RelayWithBatchBuilder<CosmosBuilder, RelayAToBTarget> for CosmosBuildComponents {
    async fn build_relay_with_batch(
        build: &CosmosBuilder,
        src_client_id: &ClientId,
        dst_client_id: &ClientId,
        src_chain: OfaChainWrapper<CosmosChain<BaseChainHandle>>,
        dst_chain: OfaChainWrapper<CosmosChain<BaseChainHandle>>,
        src_batch_sender: CosmosBatchSender,
        dst_batch_sender: CosmosBatchSender,
    ) -> Result<CosmosRelay<BaseChainHandle, BaseChainHandle>, Error> {
        let relay = build.build_relay(
            src_client_id,
            dst_client_id,
            src_chain,
            dst_chain,
            src_batch_sender,
            dst_batch_sender,
        )?;

        Ok(relay)
    }
}

#[async_trait]
impl RelayWithBatchBuilder<CosmosBuilder, RelayBToATarget> for CosmosBuildComponents {
    async fn build_relay_with_batch(
        build: &CosmosBuilder,
        src_client_id: &ClientId,
        dst_client_id: &ClientId,
        src_chain: OfaChainWrapper<CosmosChain<BaseChainHandle>>,
        dst_chain: OfaChainWrapper<CosmosChain<BaseChainHandle>>,
        src_batch_sender: CosmosBatchSender,
        dst_batch_sender: CosmosBatchSender,
    ) -> Result<CosmosRelay<BaseChainHandle, BaseChainHandle>, Error> {
        let relay = build.build_relay(
            src_client_id,
            dst_client_id,
            src_chain,
            dst_chain,
            src_batch_sender,
            dst_batch_sender,
        )?;

        Ok(relay)
    }
}
