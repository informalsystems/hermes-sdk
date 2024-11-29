use core::marker::PhantomData;

use cgp::core::error::{ErrorRaiserComponent, ErrorTypeComponent};
use cgp::core::field::impls::use_field::UseField;
use cgp::core::types::impls::WithType;
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
use hermes_relayer_components::multi::traits::birelay_at::{
    BiRelayTypeAtComponent, ProvideBiRelayTypeAt,
};
use hermes_relayer_components::multi::traits::chain_at::{ChainTypeAtComponent, HasChainTypeAt};
use hermes_relayer_components::multi::traits::relay_at::ProvideRelayTypeAt;
use hermes_relayer_components::multi::types::index::Twindex;
use hermes_test_components::driver::traits::types::builder_at::BuilderTypeAtComponent;
use hermes_test_components::driver::traits::types::chain_driver_at::ChainDriverTypeAtComponent;
use hermes_test_components::setup::binary_channel::components::*;
use hermes_test_components::setup::binary_channel::impls::fields::UseBinarySetupFields;
use hermes_test_components::setup::traits::bootstrap_at::{BootstrapAtComponent, HasBootstrapAt};
use hermes_test_components::setup::traits::builder_at::ProvideBuilderAt;
use hermes_test_components::setup::traits::create_client_options_at::{
    CreateClientMessageOptionsAtComponent, CreateClientPayloadOptionsAtComponent,
};
use hermes_test_components::setup::traits::driver::TestDriverTypeComponent;
use hermes_test_components::setup::traits::drivers::binary_channel::BinaryChannelDriverBuilder;
use hermes_test_components::setup::traits::init_channel_options_at::ProvideInitChannelOptionsAt;
use hermes_test_components::setup::traits::init_connection_options_at::ProvideInitConnectionOptionsAt;
use hermes_test_components::setup::traits::port_id_at::ProvidePortIdAt;
use ibc_relayer_types::core::ics24_host::identifier::{ChannelId, ConnectionId, PortId};

use crate::contexts::binary_channel::test_driver::CosmosBinaryChannelTestDriver;
use crate::contexts::bootstrap::CosmosBootstrap;
use crate::contexts::chain_driver::CosmosChainDriver;
use crate::contexts::relay_driver::CosmosRelayDriver;

/**
   A setup context for setting up a binary channel test driver,
   with both chains being Cosmos chains.
*/
#[derive(HasField)]
pub struct CosmosBinaryChannelSetup {
    pub bootstrap_a: CosmosBootstrap,
    pub bootstrap_b: CosmosBootstrap,
    pub create_client_payload_options: CosmosCreateClientOptions,
    pub init_connection_options: CosmosInitConnectionOptions,
    pub init_channel_options: CosmosInitChannelOptions,
    pub port_id: PortId,
}

impl CanUseBinaryChannelTestSetup for CosmosBinaryChannelSetup {}

pub struct CosmosBinaryChannelSetupComponents;

impl HasComponents for CosmosBinaryChannelSetup {
    type Components = CosmosBinaryChannelSetupComponents;
}

impl HasField<symbol!("create_client_message_options")> for CosmosBinaryChannelSetup {
    type Field = ();

    fn get_field(&self, _phantom: PhantomData<symbol!("create_client_message_options")>) -> &() {
        &()
    }
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
        ErrorTypeComponent: ProvideHermesError,
        ErrorRaiserComponent: DebugError,
        [
            BootstrapAtComponent,
            ChainTypeAtComponent,
            ChainDriverTypeAtComponent,
        ]: UseBinarySetupFields,
        CreateClientMessageOptionsAtComponent: UseField<symbol!("create_client_message_options")>,
        CreateClientPayloadOptionsAtComponent: UseField<symbol!("create_client_payload_options")>,
        TestDriverTypeComponent: WithType<CosmosBinaryChannelTestDriver>,
        BuilderTypeAtComponent: WithType<CosmosBuilder>,
        // BiRelayTypeAtComponent: WithType<CosmosBiRelay>,
    }
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

impl<const I: usize, const J: usize> ProvideRelayTypeAt<CosmosBinaryChannelSetup, I, J>
    for CosmosBinaryChannelSetupComponents
{
    type Relay = CosmosRelay;
}

impl ProvideBiRelayTypeAt<CosmosBinaryChannelSetup, 0, 1> for CosmosBinaryChannelSetupComponents {
    type BiRelay = CosmosBiRelay;
}

impl ProvideBiRelayTypeAt<CosmosBinaryChannelSetup, 1, 0> for CosmosBinaryChannelSetupComponents {
    type BiRelay = CosmosBiRelay;
}

impl ProvideBuilderAt<CosmosBinaryChannelSetup, 0, 1> for CosmosBinaryChannelSetupComponents {
    fn builder(setup: &CosmosBinaryChannelSetup) -> &CosmosBuilder {
        &setup.bootstrap_a.cosmos_builder
    }
}

impl ProvideBuilderAt<CosmosBinaryChannelSetup, 1, 0> for CosmosBinaryChannelSetupComponents {
    fn builder(setup: &CosmosBinaryChannelSetup) -> &CosmosBuilder {
        &setup.bootstrap_b.cosmos_builder
    }
}

impl ProvideInitConnectionOptionsAt<CosmosBinaryChannelSetup, 0, 1>
    for CosmosBinaryChannelSetupComponents
{
    fn init_connection_options(setup: &CosmosBinaryChannelSetup) -> CosmosInitConnectionOptions {
        setup.init_connection_options.clone()
    }
}

impl ProvideInitConnectionOptionsAt<CosmosBinaryChannelSetup, 1, 0>
    for CosmosBinaryChannelSetupComponents
{
    fn init_connection_options(setup: &CosmosBinaryChannelSetup) -> CosmosInitConnectionOptions {
        setup.init_connection_options.clone()
    }
}

impl ProvideInitChannelOptionsAt<CosmosBinaryChannelSetup, 0, 1>
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

impl ProvideInitChannelOptionsAt<CosmosBinaryChannelSetup, 1, 0>
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

impl ProvidePortIdAt<CosmosBinaryChannelSetup, 0, 1> for CosmosBinaryChannelSetupComponents {
    fn port_id_at(setup: &CosmosBinaryChannelSetup, _index: Twindex<0, 1>) -> &PortId {
        &setup.port_id
    }
}

impl ProvidePortIdAt<CosmosBinaryChannelSetup, 1, 0> for CosmosBinaryChannelSetupComponents {
    fn port_id_at(setup: &CosmosBinaryChannelSetup, _index: Twindex<1, 0>) -> &PortId {
        &setup.port_id
    }
}

pub trait CanUseCosmosBinaryChannelSetup:
    HasBootstrapAt<0, Bootstrap = CosmosBootstrap>
    + HasBootstrapAt<1, Bootstrap = CosmosBootstrap>
    + HasChainTypeAt<0, Chain = CosmosChain>
    + HasChainTypeAt<1, Chain = CosmosChain>
{
}

impl CanUseCosmosBinaryChannelSetup for CosmosBinaryChannelSetup {}
