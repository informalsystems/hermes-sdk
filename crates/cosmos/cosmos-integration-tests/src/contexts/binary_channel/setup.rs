use core::marker::PhantomData;

use cgp::core::error::{ErrorRaiserComponent, ErrorTypeProviderComponent};
use cgp::core::field::{Index, UseField, WithField};
use cgp::prelude::*;
use hermes_cosmos_chain_components::types::channel::CosmosInitChannelOptions;
use hermes_cosmos_chain_components::types::connection::CosmosInitConnectionOptions;
use hermes_cosmos_chain_components::types::payloads::client::CosmosCreateClientOptions;
use hermes_cosmos_relayer::contexts::birelay::CosmosBiRelay;
use hermes_cosmos_relayer::contexts::build::CosmosBuilder;
use hermes_cosmos_relayer::contexts::chain::CosmosChain;
use hermes_cosmos_relayer::contexts::relay::CosmosRelay;
use hermes_error::handlers::debug::DebugError;
use hermes_error::impls::UseHermesError;
use hermes_relayer_components::multi::traits::birelay_at::BiRelayTypeProviderAtComponent;
use hermes_relayer_components::multi::traits::chain_at::ChainTypeProviderAtComponent;
use hermes_relayer_components::multi::traits::relay_at::RelayTypeProviderAtComponent;
use hermes_test_components::driver::traits::types::builder_at::BuilderAtTypeProviderComponent;
use hermes_test_components::driver::traits::types::chain_driver_at::ChainDriverTypeProviderAtComponent;
use hermes_test_components::setup::binary_channel::components::*;
use hermes_test_components::setup::traits::bootstrap_at::{
    BootstrapGetterAtComponent, BootstrapTypeProviderAtComponent,
};
use hermes_test_components::setup::traits::builder_at::BuilderAtGetterComponent;
use hermes_test_components::setup::traits::create_client_options_at::{
    CreateClientMessageOptionsGetterAtComponent, CreateClientPayloadOptionsGetterAtComponent,
};
use hermes_test_components::setup::traits::driver::TestDriverTypeProviderComponent;
use hermes_test_components::setup::traits::drivers::binary_channel::BinaryChannelDriverBuilderComponent;
use hermes_test_components::setup::traits::init_channel_options_at::InitChannelOptionsGetterAtComponent;
use hermes_test_components::setup::traits::init_connection_options_at::InitConnectionOptionsGetterAtComponent;
use hermes_test_components::setup::traits::port_id_at::PortIdGetterAtComponent;
use ibc::core::host::types::identifiers::PortId;

use crate::contexts::binary_channel::test_driver::CosmosBinaryChannelTestDriver;
use crate::contexts::chain_driver::CosmosChainDriver;
use crate::impls::binary_channel_driver::BuildCosmosBinaryChannelDriver;
use crate::impls::init_channel_options::UseCosmosInitChannelOptions;

/**
   A setup context for setting up a binary channel test driver,
   with both chains being Cosmos chains.
*/
#[cgp_context(CosmosBinaryChannelSetupComponents: BinaryChannelTestComponents)]
#[derive(HasField)]
pub struct CosmosBinaryChannelSetup<BootstrapA, BootstrapB> {
    pub bootstrap_a: BootstrapA,
    pub bootstrap_b: BootstrapB,
    pub builder: CosmosBuilder,
    pub port_id: PortId,
    pub init_channel_options: CosmosInitChannelOptions,
    pub init_connection_options: CosmosInitConnectionOptions,
    pub create_client_payload_options: CosmosCreateClientOptions,
}

delegate_components! {
    CosmosBinaryChannelSetupComponents {
        ErrorTypeProviderComponent: UseHermesError,
        ErrorRaiserComponent: DebugError,
        TestDriverTypeProviderComponent:
            UseType<CosmosBinaryChannelTestDriver>,
        [
            BootstrapTypeProviderAtComponent<Index<0>>,
            BootstrapGetterAtComponent<Index<0>>,
        ]:
            WithField<symbol!("bootstrap_a")>,
        [
            BootstrapTypeProviderAtComponent<Index<1>>,
            BootstrapGetterAtComponent<Index<1>>,
        ]:
            WithField<symbol!("bootstrap_b")>,
        [
            ChainTypeProviderAtComponent<Index<0>>,
            ChainTypeProviderAtComponent<Index<1>>,
        ]:
            UseType<CosmosChain>,
        [
            ChainDriverTypeProviderAtComponent<Index<0>>,
            ChainDriverTypeProviderAtComponent<Index<1>>,
        ]: UseType<CosmosChainDriver>,
        [
            BuilderAtTypeProviderComponent<Index<0>, Index<1>>,
            BuilderAtTypeProviderComponent<Index<1>, Index<0>>,
        ]: UseType<CosmosBuilder>,
        [
            BuilderAtGetterComponent<Index<0>, Index<1>>,
            BuilderAtGetterComponent<Index<1>, Index<0>>,
        ]: UseField<symbol!("builder")>,
        [
            PortIdGetterAtComponent<Index<0>, Index<1>>,
            PortIdGetterAtComponent<Index<1>, Index<0>>,
        ]:
            UseField<symbol!("port_id")>,
        [
            InitConnectionOptionsGetterAtComponent<Index<0>, Index<1>>,
            InitConnectionOptionsGetterAtComponent<Index<1>, Index<0>>,
        ]: UseField<symbol!("init_connection_options")>,
        [
            CreateClientMessageOptionsGetterAtComponent<Index<0>, Index<1>>,
            CreateClientMessageOptionsGetterAtComponent<Index<1>, Index<0>>,
        ]: UseField<symbol!("create_client_message_options")>,
        [
            CreateClientPayloadOptionsGetterAtComponent<Index<0>, Index<1>>,
            CreateClientPayloadOptionsGetterAtComponent<Index<1>, Index<0>>,
        ]: UseField<symbol!("create_client_payload_options")>,
        InitChannelOptionsGetterAtComponent:
            UseCosmosInitChannelOptions<symbol!("init_channel_options")>,
        [
            RelayTypeProviderAtComponent<Index<0>, Index<1>>,
            RelayTypeProviderAtComponent<Index<1>, Index<0>>,
        ]: UseType<CosmosRelay>,
        BiRelayTypeProviderAtComponent<Index<0>, Index<1>>:
            UseType<CosmosBiRelay>,
        BinaryChannelDriverBuilderComponent:
            BuildCosmosBinaryChannelDriver,
    }
}

impl<BootstrapA, BootstrapB> HasField<symbol!("create_client_message_options")>
    for CosmosBinaryChannelSetup<BootstrapA, BootstrapB>
{
    type Value = ();

    fn get_field(&self, _phantom: PhantomData<symbol!("create_client_message_options")>) -> &() {
        &()
    }
}
