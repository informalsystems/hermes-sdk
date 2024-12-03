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
use hermes_relayer_components::multi::traits::birelay_at::BiRelayTypeAtComponent;
use hermes_relayer_components::multi::traits::chain_at::{ChainTypeAtComponent, HasChainTypeAt};
use hermes_relayer_components::multi::traits::relay_at::RelayTypeAtComponent;
use hermes_test_components::driver::traits::types::builder_at::BuilderTypeAtComponent;
use hermes_test_components::driver::traits::types::chain_driver_at::ChainDriverTypeAtComponent;
use hermes_test_components::setup::binary_channel::components::*;
use hermes_test_components::setup::binary_channel::impls::fields::UseBinarySetupFields;
use hermes_test_components::setup::traits::bootstrap_at::{BootstrapAtComponent, HasBootstrapAt};
use hermes_test_components::setup::traits::builder_at::BuilderAtComponent;
use hermes_test_components::setup::traits::create_client_options_at::{
    CreateClientMessageOptionsAtComponent, CreateClientPayloadOptionsAtComponent,
};
use hermes_test_components::setup::traits::driver::TestDriverTypeComponent;
use hermes_test_components::setup::traits::drivers::binary_channel::BinaryChannelDriverBuilder;
use hermes_test_components::setup::traits::init_channel_options_at::InitChannelOptionsAtComponent;
use hermes_test_components::setup::traits::init_connection_options_at::InitConnectionOptionsAtComponent;
use hermes_test_components::setup::traits::port_id_at::PortIdAtComponent;
use ibc_relayer_types::core::ics24_host::identifier::{ChannelId, ConnectionId, PortId};

use crate::contexts::binary_channel::test_driver::CosmosBinaryChannelTestDriver;
use crate::contexts::bootstrap::CosmosBootstrap;
use crate::contexts::chain_driver::CosmosChainDriver;
use crate::contexts::relay_driver::CosmosRelayDriver;
use crate::impls::init_channel_options::UseCosmosInitChannelOptions;

/**
   A setup context for setting up a binary channel test driver,
   with both chains being Cosmos chains.
*/
#[derive(HasField)]
pub struct CosmosBinaryChannelSetup {
    pub bootstrap_a: CosmosBootstrap,
    pub bootstrap_b: CosmosBootstrap,
    pub builder: CosmosBuilder,
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
        TestDriverTypeComponent: WithType<CosmosBinaryChannelTestDriver>,
        BuilderTypeAtComponent: WithType<CosmosBuilder>,
        BuilderAtComponent: UseField<symbol!("builder")>,
        PortIdAtComponent: UseField<symbol!("port_id")>,
        InitConnectionOptionsAtComponent: UseField<symbol!("init_connection_options")>,
        CreateClientMessageOptionsAtComponent: UseField<symbol!("create_client_message_options")>,
        CreateClientPayloadOptionsAtComponent: UseField<symbol!("create_client_payload_options")>,
        InitChannelOptionsAtComponent: UseCosmosInitChannelOptions,
        RelayTypeAtComponent: WithType<CosmosRelay>,
        BiRelayTypeAtComponent: WithType<CosmosBiRelay>,
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

impl HasField<symbol!("create_client_message_options")> for CosmosBinaryChannelSetup {
    type Field = ();

    fn get_field(&self, _phantom: PhantomData<symbol!("create_client_message_options")>) -> &() {
        &()
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
