use alloc::sync::Arc;

use cgp_core::error::{ErrorRaiserComponent, ErrorTypeComponent};
use cgp_core::prelude::*;
use hermes_cosmos_chain_components::types::channel::CosmosInitChannelOptions;
use hermes_cosmos_chain_components::types::connection::CosmosInitConnectionOptions;
use hermes_cosmos_relayer::contexts::birelay::CosmosBiRelay;
use hermes_cosmos_relayer::contexts::builder::CosmosBuilder;
use hermes_cosmos_relayer::contexts::chain::CosmosChain;
use hermes_cosmos_relayer::contexts::relay::CosmosRelay;
use hermes_cosmos_relayer::types::error::{DebugError, Error, ProvideCosmosError};
use hermes_test_components::driver::traits::types::birelay_at::ProvideBiRelayTypeAt;
use hermes_test_components::driver::traits::types::builder_at::ProvideBuilderTypeAt;
use hermes_test_components::driver::traits::types::chain_at::ProvideChainTypeAt;
use hermes_test_components::driver::traits::types::chain_driver_at::ProvideChainDriverTypeAt;
use hermes_test_components::driver::traits::types::relay_at::ProvideRelayTypeAt;
use hermes_test_components::setup::binary_channel::components::*;
use hermes_test_components::setup::traits::bootstrap_at::ProvideBootstrapAt;
use hermes_test_components::setup::traits::builder_at::ProvideBuilderAt;
use hermes_test_components::setup::traits::create_client_options_at::ProvideCreateClientOptionsAt;
use hermes_test_components::setup::traits::driver::ProvideTestDriverType;
use hermes_test_components::setup::traits::drivers::binary_channel::BinaryChannelDriverBuilder;
use hermes_test_components::setup::traits::init_channel_options_at::ProvideInitChannelOptionsAt;
use hermes_test_components::setup::traits::init_connection_options_at::ProvideInitConnectionOptionsAt;
use hermes_test_components::setup::traits::port_id_at::ProvidePortIdAt;
use hermes_test_components::types::index::{Index, Twindex};
use ibc_relayer::chain::client::ClientSettings;
use ibc_relayer_types::core::ics24_host::identifier::{ChannelId, ConnectionId, PortId};

use crate::contexts::binary_channel::test_driver::CosmosBinaryChannelTestDriver;
use crate::contexts::bootstrap_legacy::LegacyCosmosBootstrap;
use crate::contexts::chain_driver::CosmosChainDriver;
use crate::contexts::relay_driver::CosmosRelayDriver;

/**
   A setup context for setting up a binary channel test driver,
   with both chains being Cosmos chains.
*/
pub struct CosmosBinaryChannelSetup {
    pub bootstrap_a: Arc<LegacyCosmosBootstrap>,
    pub bootstrap_b: Arc<LegacyCosmosBootstrap>,
    pub create_client_settings: ClientSettings,
    pub init_connection_options: CosmosInitConnectionOptions,
    pub init_channel_options: CosmosInitChannelOptions,
    pub port_id: PortId,
}

impl CanUseBinaryChannelTestSetup for CosmosBinaryChannelSetup {}

pub struct CosmosBinaryChannelSetupComponents;

impl HasComponents for CosmosBinaryChannelSetup {
    type Components = CosmosBinaryChannelSetupComponents;
}

with_binary_channel_test_components! {
    delegate_components! {
        CosmosBinaryChannelSetupComponents {
            @BinaryChannelTestComponents: BinaryChannelTestComponents,
        }
    }
}

delegate_components! {
    CosmosBinaryChannelSetupComponents {
        ErrorTypeComponent: ProvideCosmosError,
        ErrorRaiserComponent: DebugError,
    }
}

impl<Setup> ProvideTestDriverType<Setup> for CosmosBinaryChannelSetupComponents
where
    Setup: Async,
{
    type TestDriver = CosmosBinaryChannelTestDriver;
}

impl BinaryChannelDriverBuilder<CosmosBinaryChannelSetup> for CosmosBinaryChannelSetupComponents {
    async fn build_driver_with_binary_channel(
        _setup: &CosmosBinaryChannelSetup,
        birelay: CosmosBiRelay,
        chain_driver_a: CosmosChainDriver,
        chain_driver_b: CosmosChainDriver,
        connection_id_a: ConnectionId,
        connection_id_b: ConnectionId,
        channel_id_a: ChannelId,
        channel_id_b: ChannelId,
        port_id_a: PortId,
        port_id_b: PortId,
    ) -> Result<CosmosBinaryChannelTestDriver, Error> {
        let relay_driver = CosmosRelayDriver { birelay };

        let driver = CosmosBinaryChannelTestDriver {
            relay_driver,
            chain_driver_a,
            chain_driver_b,
            connection_id_a,
            connection_id_b,
            channel_id_a,
            channel_id_b,
            port_id_a,
            port_id_b,
        };

        Ok(driver)
    }
}

impl<Setup, const I: usize> ProvideChainTypeAt<Setup, I> for CosmosBinaryChannelSetupComponents
where
    Setup: Async,
{
    type Chain = CosmosChain;
}

impl<const I: usize> ProvideChainDriverTypeAt<CosmosBinaryChannelSetup, I>
    for CosmosBinaryChannelSetupComponents
{
    type ChainDriver = CosmosChainDriver;
}

impl<const I: usize, const J: usize> ProvideRelayTypeAt<CosmosBinaryChannelSetup, I, J>
    for CosmosBinaryChannelSetupComponents
{
    type Relay = CosmosRelay;
}

impl<const I: usize, const J: usize> ProvideBiRelayTypeAt<CosmosBinaryChannelSetup, I, J>
    for CosmosBinaryChannelSetupComponents
{
    type BiRelay = CosmosBiRelay;
}

impl<const I: usize, const J: usize> ProvideBuilderTypeAt<CosmosBinaryChannelSetup, I, J>
    for CosmosBinaryChannelSetupComponents
{
    type Builder = CosmosBuilder;
}

impl ProvideBootstrapAt<CosmosBinaryChannelSetup, 0> for CosmosBinaryChannelSetupComponents {
    type Bootstrap = LegacyCosmosBootstrap;

    fn chain_bootstrap(
        setup: &CosmosBinaryChannelSetup,
        _index: Index<0>,
    ) -> &LegacyCosmosBootstrap {
        &setup.bootstrap_a
    }
}

impl ProvideBootstrapAt<CosmosBinaryChannelSetup, 1> for CosmosBinaryChannelSetupComponents {
    type Bootstrap = LegacyCosmosBootstrap;

    fn chain_bootstrap(
        setup: &CosmosBinaryChannelSetup,
        _index: Index<1>,
    ) -> &LegacyCosmosBootstrap {
        &setup.bootstrap_b
    }
}

impl ProvideBuilderAt<CosmosBinaryChannelSetup, 0, 1> for CosmosBinaryChannelSetupComponents {
    fn builder(setup: &CosmosBinaryChannelSetup) -> &CosmosBuilder {
        &setup.bootstrap_a.builder
    }
}

impl ProvideBuilderAt<CosmosBinaryChannelSetup, 1, 0> for CosmosBinaryChannelSetupComponents {
    fn builder(setup: &CosmosBinaryChannelSetup) -> &CosmosBuilder {
        &setup.bootstrap_b.builder
    }
}

impl<const I: usize, const J: usize> ProvideCreateClientOptionsAt<CosmosBinaryChannelSetup, I, J>
    for CosmosBinaryChannelSetupComponents
{
    fn create_client_payload_options(
        setup: &CosmosBinaryChannelSetup,
        _index: Twindex<I, J>,
    ) -> &ClientSettings {
        &setup.create_client_settings
    }

    fn create_client_message_options(
        _setup: &CosmosBinaryChannelSetup,
        _index: Twindex<I, J>,
    ) -> &() {
        &()
    }
}

impl<const I: usize, const J: usize> ProvideInitConnectionOptionsAt<CosmosBinaryChannelSetup, I, J>
    for CosmosBinaryChannelSetupComponents
{
    fn init_connection_options(setup: &CosmosBinaryChannelSetup) -> CosmosInitConnectionOptions {
        setup.init_connection_options.clone()
    }
}

impl<const I: usize, const J: usize> ProvideInitChannelOptionsAt<CosmosBinaryChannelSetup, I, J>
    for CosmosBinaryChannelSetupComponents
{
    fn init_channel_options(
        setup: &CosmosBinaryChannelSetup,
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

impl<const I: usize, const J: usize> ProvidePortIdAt<CosmosBinaryChannelSetup, I, J>
    for CosmosBinaryChannelSetupComponents
{
    fn port_id_at(setup: &CosmosBinaryChannelSetup, _index: Twindex<I, J>) -> &PortId {
        &setup.port_id
    }
}
