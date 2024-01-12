use cgp_core::delegate_all;
use cgp_core::prelude::*;
use cgp_core::ErrorRaiserComponent;
use cgp_core::ErrorTypeComponent;
use cgp_error_eyre::ProvideEyreError;
use cgp_error_eyre::RaiseDebugError;
use eyre::Error;
use hermes_cosmos_client_components::types::channel::CosmosInitChannelOptions;
use hermes_cosmos_client_components::types::connection::CosmosInitConnectionOptions;
use hermes_cosmos_relayer::contexts::birelay::CosmosBiRelay;
use hermes_cosmos_relayer::contexts::builder::CosmosBuilder;
use hermes_cosmos_relayer::contexts::chain::CosmosChain;
use hermes_cosmos_relayer::contexts::relay::CosmosRelay;
use hermes_test_components::driver::traits::types::birelay_at::ProvideBiRelayTypeAt;
use hermes_test_components::driver::traits::types::builder_at::ProvideBuilderTypeAt;
use hermes_test_components::driver::traits::types::chain_at::ProvideChainTypeAt;
use hermes_test_components::driver::traits::types::chain_driver_at::ProvideChainDriverTypeAt;
use hermes_test_components::driver::traits::types::relay_at::ProvideRelayTypeAt;
use hermes_test_components::setup::components::binary_channel::BinaryChannelTestComponents;
use hermes_test_components::setup::components::binary_channel::CanUseBinaryChannelTestSetup;
use hermes_test_components::setup::components::binary_channel::IsBinaryChannelTestComponent;
use hermes_test_components::setup::traits::bootstrap_at::ProvideBootstrapAt;
use hermes_test_components::setup::traits::builder_at::ProvideBuilderAt;
use hermes_test_components::setup::traits::create_client_options_at::ProvideCreateClientOptionsAt;
use hermes_test_components::setup::traits::driver::ProvideDriverType;
use hermes_test_components::setup::traits::drivers::binary_channel::BinaryChannelDriverBuilder;
use hermes_test_components::setup::traits::init_channel_options_at::ProvideInitChannelOptionsAt;
use hermes_test_components::setup::traits::init_connection_options_at::ProvideInitConnectionOptionsAt;
use hermes_test_components::setup::traits::port_id_at::ProvidePortIdAt;
use hermes_test_components::types::index::Index;
use hermes_test_components::types::index::Twindex;
use ibc_relayer::chain::client::ClientSettings;
use ibc_relayer_types::core::ics24_host::identifier::ChannelId;
use ibc_relayer_types::core::ics24_host::identifier::ConnectionId;
use ibc_relayer_types::core::ics24_host::identifier::PortId;

use crate::contexts::bootstrap::CosmosBootstrap;
use crate::contexts::chain::CosmosChainDriver;

pub struct CosmosSetup {
    pub bootstrap: CosmosBootstrap,
    pub create_client_settings: ClientSettings,
    pub init_connection_options: CosmosInitConnectionOptions,
    pub init_channel_options: CosmosInitChannelOptions,
    pub port_id: PortId,
}

impl CanUseBinaryChannelTestSetup for CosmosSetup {}

pub struct CosmosSetupComponents;

impl HasComponents for CosmosSetup {
    type Components = CosmosSetupComponents;
}

delegate_all!(
    IsBinaryChannelTestComponent,
    BinaryChannelTestComponents,
    CosmosSetupComponents,
);

delegate_components! {
    CosmosSetupComponents {
        ErrorTypeComponent: ProvideEyreError,
        ErrorRaiserComponent: RaiseDebugError,
    }
}

impl<Setup> ProvideDriverType<Setup> for CosmosSetupComponents
where
    Setup: Async,
{
    type Driver = ();
}

impl BinaryChannelDriverBuilder<CosmosSetup> for CosmosSetupComponents {
    async fn build_driver_with_binary_channel(
        _setup: &CosmosSetup,
        _birelay: CosmosBiRelay,
        _connection_id_a: ConnectionId,
        _connection_id_b: ConnectionId,
        _channel_id_a: ChannelId,
        _channel_id_b: ChannelId,
    ) -> Result<(), Error> {
        Ok(())
    }
}

impl<Setup, const I: usize> ProvideChainTypeAt<Setup, I> for CosmosSetupComponents
where
    Setup: Async,
{
    type Chain = CosmosChain;
}

impl<const I: usize> ProvideChainDriverTypeAt<CosmosSetup, I> for CosmosSetupComponents {
    type ChainDriver = CosmosChainDriver;
}

impl<const I: usize, const J: usize> ProvideRelayTypeAt<CosmosSetup, I, J>
    for CosmosSetupComponents
{
    type Relay = CosmosRelay;
}

impl<const I: usize, const J: usize> ProvideBiRelayTypeAt<CosmosSetup, I, J>
    for CosmosSetupComponents
{
    type BiRelay = CosmosBiRelay;
}

impl<const I: usize, const J: usize> ProvideBuilderTypeAt<CosmosSetup, I, J>
    for CosmosSetupComponents
{
    type Builder = CosmosBuilder;
}

impl<const I: usize> ProvideBootstrapAt<CosmosSetup, I> for CosmosSetupComponents {
    type Bootstrap = CosmosBootstrap;

    fn chain_bootstrap(setup: &CosmosSetup, _index: Index<I>) -> &CosmosBootstrap {
        &setup.bootstrap
    }
}

impl<const I: usize, const J: usize> ProvideBuilderAt<CosmosSetup, I, J> for CosmosSetupComponents {
    fn builder(setup: &CosmosSetup) -> &CosmosBuilder {
        &setup.bootstrap.builder
    }
}

impl<const I: usize, const J: usize> ProvideCreateClientOptionsAt<CosmosSetup, I, J>
    for CosmosSetupComponents
{
    fn create_client_options(setup: &CosmosSetup, _index: Twindex<I, J>) -> &ClientSettings {
        &setup.create_client_settings
    }
}

impl<const I: usize, const J: usize> ProvideInitConnectionOptionsAt<CosmosSetup, I, J>
    for CosmosSetupComponents
{
    fn init_connection_options(setup: &CosmosSetup) -> CosmosInitConnectionOptions {
        setup.init_connection_options.clone()
    }
}

impl<const I: usize, const J: usize> ProvideInitChannelOptionsAt<CosmosSetup, I, J>
    for CosmosSetupComponents
{
    fn init_channel_options(
        setup: &CosmosSetup,
        connection_id: &ConnectionId,
        _counterparty_connection_id: &ConnectionId,
    ) -> CosmosInitChannelOptions {
        let mut options = setup.init_channel_options.clone();

        // Use an init channel options that is provided by the setup.
        // Insert the connection ID to the front (or to the back?) to allow
        // testing multihop connections in the future.
        options.connection_hops.insert(0, connection_id.clone());

        options
    }
}

impl<const I: usize, const J: usize> ProvidePortIdAt<CosmosSetup, I, J> for CosmosSetupComponents {
    fn port_id_at(setup: &CosmosSetup, _index: Twindex<I, J>) -> &PortId {
        &setup.port_id
    }
}
