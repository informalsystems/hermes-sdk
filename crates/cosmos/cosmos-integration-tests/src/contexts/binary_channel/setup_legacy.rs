use alloc::sync::Arc;

use cgp::core::error::{ErrorRaiserComponent, ErrorTypeComponent};
use cgp::prelude::*;
use hermes_cosmos_chain_components::types::channel::CosmosInitChannelOptions;
use hermes_cosmos_chain_components::types::connection::CosmosInitConnectionOptions;
use hermes_cosmos_chain_components::types::payloads::client::CosmosCreateClientOptions;
use hermes_cosmos_relayer::contexts::birelay::CosmosBiRelay;
use hermes_cosmos_relayer::contexts::build::CosmosBuilder;
use hermes_cosmos_relayer::contexts::chain::CosmosChain;
use hermes_cosmos_relayer::contexts::relay::CosmosRelay;
use hermes_error::handlers::debug::DebugError;
use hermes_error::impls::ProvideHermesError;
use hermes_error::types::Error;
use hermes_relayer_components::multi::traits::birelay_at::ProvideBiRelayTypeAt;
use hermes_relayer_components::multi::traits::chain_at::ProvideChainTypeAt;
use hermes_relayer_components::multi::traits::relay_at::ProvideRelayTypeAt;
use hermes_relayer_components::multi::types::index::{Index, Twindex};
use hermes_test_components::driver::traits::types::builder_at::ProvideBuilderTypeAt;
use hermes_test_components::driver::traits::types::chain_driver_at::ProvideChainDriverTypeAt;
use hermes_test_components::setup::binary_channel::components::*;
use hermes_test_components::setup::traits::bootstrap_at::ProvideBootstrapAt;
use hermes_test_components::setup::traits::builder_at::ProvideBuilderAt;
use hermes_test_components::setup::traits::create_client_options_at::{
    ProvideCreateClientMessageOptionsAt, ProvideCreateClientPayloadOptionsAt,
};
use hermes_test_components::setup::traits::driver::ProvideTestDriverType;
use hermes_test_components::setup::traits::drivers::binary_channel::BinaryChannelDriverBuilder;
use hermes_test_components::setup::traits::init_channel_options_at::ProvideInitChannelOptionsAt;
use hermes_test_components::setup::traits::init_connection_options_at::ProvideInitConnectionOptionsAt;
use hermes_test_components::setup::traits::port_id_at::ProvidePortIdAt;
use ibc_relayer_types::core::ics24_host::identifier::{ChannelId, ConnectionId, PortId};

use crate::contexts::binary_channel::test_driver::CosmosBinaryChannelTestDriver;
use crate::contexts::bootstrap_legacy::LegacyCosmosBootstrap;
use crate::contexts::chain_driver::CosmosChainDriver;
use crate::contexts::relay_driver::CosmosRelayDriver;

/**
   A setup context for setting up a binary channel test driver,
   with both chains being Cosmos chains.
*/
pub struct LegacyCosmosBinaryChannelSetup {
    pub bootstrap_a: Arc<LegacyCosmosBootstrap>,
    pub bootstrap_b: Arc<LegacyCosmosBootstrap>,
    pub create_client_settings: CosmosCreateClientOptions,
    pub init_connection_options: CosmosInitConnectionOptions,
    pub init_channel_options: CosmosInitChannelOptions,
    pub port_id: PortId,
}

impl CanUseBinaryChannelTestSetup for LegacyCosmosBinaryChannelSetup {}

pub struct LegacyCosmosBinaryChannelSetupComponents;

impl HasComponents for LegacyCosmosBinaryChannelSetup {
    type Components = LegacyCosmosBinaryChannelSetupComponents;
}

with_binary_channel_test_components! {
    delegate_components! {
        LegacyCosmosBinaryChannelSetupComponents {
            @BinaryChannelTestComponents: BinaryChannelTestComponents,
        }
    }
}

delegate_components! {
    LegacyCosmosBinaryChannelSetupComponents {
        ErrorTypeComponent: ProvideHermesError,
        ErrorRaiserComponent: DebugError,
    }
}

impl<Setup> ProvideTestDriverType<Setup> for LegacyCosmosBinaryChannelSetupComponents
where
    Setup: Async,
{
    type TestDriver = CosmosBinaryChannelTestDriver;
}

impl BinaryChannelDriverBuilder<LegacyCosmosBinaryChannelSetup>
    for LegacyCosmosBinaryChannelSetupComponents
{
    async fn build_driver_with_binary_channel(
        _setup: &LegacyCosmosBinaryChannelSetup,
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

impl<Setup, const I: usize> ProvideChainTypeAt<Setup, I>
    for LegacyCosmosBinaryChannelSetupComponents
where
    Setup: Async,
{
    type Chain = CosmosChain;
}

impl<const I: usize> ProvideChainDriverTypeAt<LegacyCosmosBinaryChannelSetup, I>
    for LegacyCosmosBinaryChannelSetupComponents
{
    type ChainDriver = CosmosChainDriver;
}

impl<const I: usize, const J: usize> ProvideRelayTypeAt<LegacyCosmosBinaryChannelSetup, I, J>
    for LegacyCosmosBinaryChannelSetupComponents
{
    type Relay = CosmosRelay;
}

impl ProvideBiRelayTypeAt<LegacyCosmosBinaryChannelSetup, 0, 1>
    for LegacyCosmosBinaryChannelSetupComponents
{
    type BiRelay = CosmosBiRelay;
}

impl ProvideBiRelayTypeAt<LegacyCosmosBinaryChannelSetup, 1, 0>
    for LegacyCosmosBinaryChannelSetupComponents
{
    type BiRelay = CosmosBiRelay;
}

impl ProvideBuilderTypeAt<LegacyCosmosBinaryChannelSetup, 0, 1>
    for LegacyCosmosBinaryChannelSetupComponents
{
    type Builder = CosmosBuilder;
}

impl ProvideBuilderTypeAt<LegacyCosmosBinaryChannelSetup, 1, 0>
    for LegacyCosmosBinaryChannelSetupComponents
{
    type Builder = CosmosBuilder;
}

impl ProvideBootstrapAt<LegacyCosmosBinaryChannelSetup, 0>
    for LegacyCosmosBinaryChannelSetupComponents
{
    type Bootstrap = LegacyCosmosBootstrap;

    fn chain_bootstrap(
        setup: &LegacyCosmosBinaryChannelSetup,
        _index: Index<0>,
    ) -> &LegacyCosmosBootstrap {
        &setup.bootstrap_a
    }
}

impl ProvideBootstrapAt<LegacyCosmosBinaryChannelSetup, 1>
    for LegacyCosmosBinaryChannelSetupComponents
{
    type Bootstrap = LegacyCosmosBootstrap;

    fn chain_bootstrap(
        setup: &LegacyCosmosBinaryChannelSetup,
        _index: Index<1>,
    ) -> &LegacyCosmosBootstrap {
        &setup.bootstrap_b
    }
}

impl ProvideBuilderAt<LegacyCosmosBinaryChannelSetup, 0, 1>
    for LegacyCosmosBinaryChannelSetupComponents
{
    fn builder(setup: &LegacyCosmosBinaryChannelSetup) -> &CosmosBuilder {
        &setup.bootstrap_a.cosmos_builder
    }
}

impl ProvideBuilderAt<LegacyCosmosBinaryChannelSetup, 1, 0>
    for LegacyCosmosBinaryChannelSetupComponents
{
    fn builder(setup: &LegacyCosmosBinaryChannelSetup) -> &CosmosBuilder {
        &setup.bootstrap_b.cosmos_builder
    }
}

impl<const I: usize, const J: usize>
    ProvideCreateClientPayloadOptionsAt<LegacyCosmosBinaryChannelSetup, I, J>
    for LegacyCosmosBinaryChannelSetupComponents
{
    fn create_client_payload_options(
        setup: &LegacyCosmosBinaryChannelSetup,
        _index: Twindex<I, J>,
    ) -> &CosmosCreateClientOptions {
        &setup.create_client_settings
    }
}

impl<const I: usize, const J: usize>
    ProvideCreateClientMessageOptionsAt<LegacyCosmosBinaryChannelSetup, I, J>
    for LegacyCosmosBinaryChannelSetupComponents
{
    fn create_client_message_options(
        _setup: &LegacyCosmosBinaryChannelSetup,
        _index: Twindex<I, J>,
    ) -> &() {
        &()
    }
}

impl<const I: usize, const J: usize>
    ProvideInitConnectionOptionsAt<LegacyCosmosBinaryChannelSetup, I, J>
    for LegacyCosmosBinaryChannelSetupComponents
{
    fn init_connection_options(
        setup: &LegacyCosmosBinaryChannelSetup,
    ) -> CosmosInitConnectionOptions {
        setup.init_connection_options.clone()
    }
}

impl<const I: usize, const J: usize>
    ProvideInitChannelOptionsAt<LegacyCosmosBinaryChannelSetup, I, J>
    for LegacyCosmosBinaryChannelSetupComponents
{
    fn init_channel_options(
        setup: &LegacyCosmosBinaryChannelSetup,
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

impl<const I: usize, const J: usize> ProvidePortIdAt<LegacyCosmosBinaryChannelSetup, I, J>
    for LegacyCosmosBinaryChannelSetupComponents
{
    fn port_id_at(setup: &LegacyCosmosBinaryChannelSetup, _index: Twindex<I, J>) -> &PortId {
        &setup.port_id
    }
}
