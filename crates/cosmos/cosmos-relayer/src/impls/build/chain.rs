use hermes_error::types::Error;
use hermes_relayer_components::build::traits::components::chain_builder::ChainBuilder;
use hermes_relayer_components::build::traits::target::chain::{ChainATarget, ChainBTarget};
use ibc_relayer_types::core::ics24_host::identifier::ChainId;

use crate::contexts::builder::CosmosBuilder;
use crate::contexts::chain::CosmosChain;
use crate::impls::build::components::CosmosBaseBuildComponents;

impl ChainBuilder<CosmosBuilder, ChainATarget> for CosmosBaseBuildComponents {
    async fn build_chain(
        build: &CosmosBuilder,
        _target: ChainATarget,
        chain_id: &ChainId,
    ) -> Result<CosmosChain, Error> {
        let chain = build.build_chain(chain_id).await?;

        Ok(chain)
    }
}

impl ChainBuilder<CosmosBuilder, ChainBTarget> for CosmosBaseBuildComponents {
    async fn build_chain(
        build: &CosmosBuilder,
        _target: ChainBTarget,
        chain_id: &ChainId,
    ) -> Result<CosmosChain, Error> {
        let chain = build.build_chain(chain_id).await?;

        Ok(chain)
    }
}
