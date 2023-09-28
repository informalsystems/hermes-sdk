use async_trait::async_trait;
use ibc_relayer::chain::handle::BaseChainHandle;
use ibc_relayer_all_in_one::one_for_all::types::relay::OfaRelayWrapper;
use ibc_relayer_components::build::traits::components::birelay_from_relay_builder::BiRelayFromRelayBuilder;

use crate::contexts::birelay::CosmosBiRelay;
use crate::contexts::builder::CosmosBuilder;
use crate::contexts::relay::CosmosRelay;
use crate::impls::build::components::CosmosBuildComponents;
use crate::types::error::Error;

#[async_trait]
impl BiRelayFromRelayBuilder<CosmosBuilder> for CosmosBuildComponents {
    async fn build_birelay_from_relays(
        build: &CosmosBuilder,
        relay_a_to_b: CosmosRelay<BaseChainHandle, BaseChainHandle>,
        relay_b_to_a: CosmosRelay<BaseChainHandle, BaseChainHandle>,
    ) -> Result<CosmosBiRelay<BaseChainHandle, BaseChainHandle>, Error> {
        let birelay = CosmosBiRelay {
            runtime: build.runtime.clone(),
            relay_a_to_b: OfaRelayWrapper::new(relay_a_to_b),
            relay_b_to_a: OfaRelayWrapper::new(relay_b_to_a),
        };

        Ok(birelay)
    }
}
