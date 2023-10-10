use async_trait::async_trait;
use ibc_relayer::chain::handle::BaseChainHandle;
use ibc_relayer_components::build::traits::components::chain_builder::ChainBuilder;
use ibc_relayer_components::build::traits::target::chain::{ChainATarget, ChainBTarget};
use ibc_relayer_types::core::ics24_host::identifier::ChainId;

use crate::contexts::builder::CosmosBuilder;
use crate::contexts::chain::CosmosChain;
use crate::impls::build::components::CosmosBuildComponents;
use crate::types::error::Error;

#[async_trait]
impl ChainBuilder<CosmosBuilder, ChainATarget> for CosmosBuildComponents {
    async fn build_chain(
        build: &CosmosBuilder,
        _target: ChainATarget,
        chain_id: &ChainId,
    ) -> Result<CosmosChain<BaseChainHandle>, Error> {
        let chain = build.build_chain(chain_id).await?;

        Ok(chain)
    }
}

#[async_trait]
impl ChainBuilder<CosmosBuilder, ChainBTarget> for CosmosBuildComponents {
    async fn build_chain(
        build: &CosmosBuilder,
        _target: ChainBTarget,
        chain_id: &ChainId,
    ) -> Result<CosmosChain<BaseChainHandle>, Error> {
        let chain = build.build_chain(chain_id).await?;

        Ok(chain)
    }
}
