use cgp::core::error::{ErrorRaiserComponent, ErrorTypeProviderComponent};
use cgp::core::field::{Index, WithField};
use cgp::core::types::WithType;
use cgp::extra::run::CanRun;
use cgp::prelude::*;
use hermes_cosmos_relayer::contexts::birelay::CosmosBiRelay;
use hermes_cosmos_relayer::contexts::chain::CosmosChain;
use hermes_cosmos_relayer::contexts::relay::CosmosRelay;
use hermes_error::handlers::debug::DebugError;
use hermes_error::impls::UseHermesError;
use hermes_error::types::Error;
use hermes_relayer_components::multi::traits::birelay_at::{
    BiRelayAtTypeProviderComponent, BiRelayGetterAtComponent,
};
use hermes_relayer_components::multi::traits::chain_at::ChainTypeAtComponent;
use hermes_relayer_components::multi::traits::relay_at::RelayAtTypeProviderComponent;
use hermes_test_components::relay_driver::run::{
    RelayerBackgroundRunner, RelayerBackgroundRunnerComponent,
};
use tokio::task::AbortHandle;

#[cgp_context(CosmosRelayDriverComponents)]
#[derive(HasField)]
pub struct CosmosRelayDriver {
    pub birelay: CosmosBiRelay,
}

delegate_components! {
    CosmosRelayDriverComponents {
        ErrorTypeProviderComponent: UseHermesError,
        ErrorRaiserComponent: DebugError,
        [
            ChainTypeAtComponent<Index<0>>,
            ChainTypeAtComponent<Index<1>>,
        ]:
            WithType<CosmosChain>,
        [
            RelayAtTypeProviderComponent<Index<0>, Index<1>>,
            RelayAtTypeProviderComponent<Index<1>, Index<0>>,
        ]: WithType<CosmosRelay>,
        BiRelayAtTypeProviderComponent<Index<0>, Index<1>>:
            WithType<CosmosBiRelay>,
        BiRelayGetterAtComponent<Index<0>, Index<1>>:
            WithField<symbol!("birelay")>,
    }
}

#[cgp_provider(RelayerBackgroundRunnerComponent)]
impl RelayerBackgroundRunner<CosmosRelayDriver> for CosmosRelayDriverComponents {
    type RunHandle<'a> = AbortOnDrop;

    async fn run_relayer_in_background(
        relay_driver: &CosmosRelayDriver,
    ) -> Result<AbortOnDrop, Error> {
        let birelay = relay_driver.birelay.clone();

        let handle = tokio::spawn(async move {
            let _ = birelay.run().await;
        });

        Ok(AbortOnDrop(handle.abort_handle()))
    }
}

pub struct AbortOnDrop(pub AbortHandle);

impl Drop for AbortOnDrop {
    fn drop(&mut self) {
        self.0.abort();
    }
}

pub trait CanUseCosmosRelayDriver:
    CanUseComponent<BiRelayAtTypeProviderComponent<Index<0>, Index<1>>, (Index<0>, Index<1>)>
    + CanUseComponent<BiRelayGetterAtComponent<Index<0>, Index<1>>, (Index<0>, Index<1>)>
{
}

impl CanUseCosmosRelayDriver for CosmosRelayDriver {}
