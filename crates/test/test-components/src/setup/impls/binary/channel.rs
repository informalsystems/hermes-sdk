use cgp_core::prelude::*;
use cgp_core::CanRaiseError;
use hermes_relayer_components::build::traits::components::birelay_from_relay_builder::CanBuildBiRelayFromRelays;
use hermes_relayer_components::build::traits::components::relay_from_chains_builder::CanBuildRelayFromChains;
use hermes_relayer_components::build::traits::target::relay::RelayAToBTarget;
use hermes_relayer_components::build::traits::target::relay::RelayBToATarget;
use hermes_relayer_components::chain::traits::types::create_client::HasCreateClientOptionsType;
use hermes_relayer_components::chain::traits::types::ibc::HasIbcChainTypes;
use hermes_relayer_components::relay::traits::components::client_creator::CanCreateClient;
use hermes_relayer_components::relay::traits::target::DestinationTarget;
use hermes_relayer_components::relay::traits::target::SourceTarget;

use crate::bootstrap::traits::chain::CanBootstrapChain;
use crate::driver::traits::types::builder_at::HasBuilderTypeAt;
use crate::driver::traits::types::chain_at::ChainTypeAt;
use crate::driver::traits::types::chain_at::HasChainTypeAt;
use crate::driver::traits::types::relay_at::HasRelayTypeAt;
use crate::driver::traits::types::relay_at::RelayTypeAt;
use crate::setup::traits::bootstrap_at::BootstrapAt;
use crate::setup::traits::bootstrap_at::HasBootstrapAt;
use crate::setup::traits::builder_at::HasBuilderAt;
use crate::setup::traits::create_client_options_at::HasCreateClientOptionsAt;
use crate::setup::traits::driver::DriverBuilder;
use crate::setup::traits::driver::HasDriverType;

pub struct SetupBinaryChannelDriver<const A: usize, const B: usize>;

impl<Setup, Driver, const A: usize, const B: usize> DriverBuilder<Setup>
    for SetupBinaryChannelDriver<A, B>
where
    Setup: HasDriverType<Driver = Driver>
        + HasBootstrapAt<A>
        + HasBootstrapAt<B>
        + HasBuilderAt<A, B>
        + HasCreateClientOptionsAt<A, B>
        + HasCreateClientOptionsAt<B, A>
        + CanRaiseError<<BootstrapAt<Setup, A> as HasErrorType>::Error>
        + CanRaiseError<<BootstrapAt<Setup, B> as HasErrorType>::Error>
        + CanRaiseError<<RelayTypeAt<Driver, A, B> as HasErrorType>::Error>
        + CanRaiseError<<Driver::Builder as HasErrorType>::Error>,
    Driver: HasErrorType
        + HasChainTypeAt<A>
        + HasChainTypeAt<B>
        + HasRelayTypeAt<A, B>
        + HasBuilderTypeAt<A, B>,
    BootstrapAt<Setup, A>: CanBootstrapChain,
    BootstrapAt<Setup, B>: CanBootstrapChain,
    Driver::Builder: CanBuildRelayFromChains<RelayAToBTarget>
        + CanBuildRelayFromChains<RelayBToATarget>
        + CanBuildBiRelayFromRelays,
    ChainTypeAt<Driver, A>: HasIbcChainTypes<ChainTypeAt<Driver, B>>
        + HasCreateClientOptionsType<ChainTypeAt<Driver, B>>
        + Clone,
    ChainTypeAt<Driver, B>: HasIbcChainTypes<ChainTypeAt<Driver, A>>
        + HasCreateClientOptionsType<ChainTypeAt<Driver, A>>
        + Clone,
    RelayTypeAt<Driver, A, B>: CanCreateClient<SourceTarget> + CanCreateClient<DestinationTarget>,
{
    async fn build_driver(setup: &Setup) -> Result<Setup::Driver, Setup::Error> {
        let bootstrap_a = <Setup as HasBootstrapAt<A>>::chain_bootstrap(setup);

        let chain_a = bootstrap_a
            .bootstrap_chain("chain-a")
            .await
            .map_err(Setup::raise_error)?;

        let bootstrap_b = <Setup as HasBootstrapAt<B>>::chain_bootstrap(setup);

        let chain_b = bootstrap_b
            .bootstrap_chain("chain-b")
            .await
            .map_err(Setup::raise_error)?;

        let client_id_a = <RelayTypeAt<Driver, A, B>>::create_client(
            SourceTarget,
            &chain_a,
            &chain_b,
            <Setup as HasCreateClientOptionsAt<B, A>>::create_client_options(setup),
        )
        .await
        .map_err(Setup::raise_error)?;

        let client_id_b = <RelayTypeAt<Driver, A, B>>::create_client(
            DestinationTarget,
            &chain_b,
            &chain_a,
            <Setup as HasCreateClientOptionsAt<A, B>>::create_client_options(setup),
        )
        .await
        .map_err(Setup::raise_error)?;

        let builder = setup.builder();

        let relay_a_to_b = builder
            .build_relay_from_chains(
                RelayAToBTarget,
                &client_id_a,
                &client_id_b,
                chain_a.clone(),
                chain_b.clone(),
            )
            .await
            .map_err(Setup::raise_error)?;

        let relay_b_to_a = builder
            .build_relay_from_chains(
                RelayBToATarget,
                &client_id_b,
                &client_id_a,
                chain_b.clone(),
                chain_a.clone(),
            )
            .await
            .map_err(Setup::raise_error)?;

        let birelay = builder
            .build_birelay_from_relays(relay_a_to_b, relay_b_to_a)
            .await
            .map_err(Setup::raise_error)?;

        todo!()
    }
}
