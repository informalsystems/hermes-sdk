use cgp::core::error::{ErrorRaiserComponent, ErrorTypeProviderComponent};
use cgp::core::field::{Index, UseField, WithField};
use hermes_core::relayer_components::multi::traits::birelay_at::BiRelayTypeProviderAtComponent;
use hermes_core::relayer_components::multi::traits::chain_at::ChainTypeProviderAtComponent;
use hermes_core::relayer_components::multi::traits::relay_at::RelayTypeProviderAtComponent;
use hermes_core::test_components::driver::traits::{
    BuilderAtTypeProviderComponent, ChainDriverTypeProviderAtComponent,
};
use hermes_core::test_components::setup::binary_channel::*;
use hermes_core::test_components::setup::traits::{
    BinaryChannelDriverBuilderComponent, BootstrapGetterAtComponent,
    BootstrapTypeProviderAtComponent, BuilderAtGetterComponent,
    CreateClientMessageOptionsGetterAtComponent, CreateClientPayloadOptionsGetterAtComponent,
    InitChannelOptionsGetterAtComponent, InitConnectionOptionsGetterAtComponent,
    PortIdGetterAtComponent, TestDriverTypeProviderComponent,
};
use hermes_cosmos_chain_components::types::{
    CosmosCreateClientOptions, CosmosInitChannelOptions, CosmosInitConnectionOptions,
};
use hermes_cosmos_relayer::contexts::{CosmosBiRelay, CosmosBuilder, CosmosChain, CosmosRelay};
use hermes_error::handlers::DebugError;
use hermes_error::impls::UseHermesError;
use hermes_prelude::*;
use ibc::core::host::types::identifiers::PortId;

use crate::contexts::{CosmosBinaryChannelTestDriver, CosmosChainDriver};
use crate::impls::{BuildCosmosBinaryChannelDriver, UseCosmosInitChannelOptions};

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
    pub create_client_message_options: (),
}

impl<BootstrapA, BootstrapB> CosmosBinaryChannelSetup<BootstrapA, BootstrapB> {
    pub fn new_with_defaults(
        bootstrap_a: BootstrapA,
        bootstrap_b: BootstrapB,
        builder: CosmosBuilder,
    ) -> Self {
        Self {
            bootstrap_a,
            bootstrap_b,
            builder,
            create_client_payload_options: Default::default(),
            create_client_message_options: Default::default(),
            init_connection_options: Default::default(),
            init_channel_options: Default::default(),
            port_id: PortId::transfer(),
        }
    }
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
        BuilderAtTypeProviderComponent<Index<0>, Index<1>>:
            UseType<CosmosBuilder>,
        BuilderAtGetterComponent<Index<0>, Index<1>>:
            UseField<symbol!("builder")>,
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
        [
            InitChannelOptionsGetterAtComponent<Index<0>, Index<1>>,
            InitChannelOptionsGetterAtComponent<Index<1>, Index<0>>,
        ]:
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
