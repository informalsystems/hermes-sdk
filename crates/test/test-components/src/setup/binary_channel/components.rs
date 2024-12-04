use cgp::core::error::ErrorRaiser;
use cgp::prelude::*;
use hermes_relayer_components::build::traits::builders::birelay_from_relay_builder::CanBuildBiRelayFromRelays;
use hermes_relayer_components::build::traits::builders::relay_from_chains_builder::CanBuildRelayFromChains;
use hermes_relayer_components::chain::traits::types::channel::HasInitChannelOptionsType;
use hermes_relayer_components::chain::traits::types::connection::HasInitConnectionOptionsType;
use hermes_relayer_components::chain::traits::types::create_client::{
    HasCreateClientMessageOptionsType, HasCreateClientPayloadOptionsType,
};
use hermes_relayer_components::chain::traits::types::ibc::HasIbcChainTypes;
use hermes_relayer_components::chain::types::aliases::PortIdOf;
use hermes_relayer_components::multi::traits::birelay_at::HasBiRelayTypeAt;
use hermes_relayer_components::multi::traits::chain_at::HasChainTypeAt;
use hermes_relayer_components::multi::traits::relay_at::{HasRelayTypeAt, RelayAt};
use hermes_relayer_components::multi::types::index::Index;
use hermes_relayer_components::relay::impls::channel::bootstrap::CanBootstrapChannel;
use hermes_relayer_components::relay::impls::connection::bootstrap::CanBootstrapConnection;
use hermes_relayer_components::relay::traits::chains::{
    CanRaiseRelayChainErrors, HasRelayChains, HasRelayClientIds,
};
use hermes_relayer_components::relay::traits::client_creator::CanCreateClient;
use hermes_relayer_components::relay::traits::target::{
    DestinationTarget, HasDestinationTargetChainTypes, HasSourceTargetChainTypes, HasTargetChains,
    SourceTarget,
};

use crate::bootstrap::traits::chain::CanBootstrapChain;
use crate::chain_driver::traits::types::chain::HasChain;
use crate::driver::traits::types::builder_at::ProvideBuilderTypeAt;
use crate::driver::traits::types::chain_driver_at::HasChainDriverTypeAt;
use crate::setup::binary_channel::impls::setup::SetupBinaryChannelDriver;
use crate::setup::impls::birelay::SetupBiRelayWithBuilder;
use crate::setup::impls::chain::SetupChainWithBootstrap;
use crate::setup::impls::channel::SetupChannelHandshake;
use crate::setup::impls::clients::SetupClientsWithRelay;
use crate::setup::impls::connection::SetupConnectionHandshake;
use crate::setup::impls::relay::SetupRelayWithBuilder;
use crate::setup::impls::run_test::BuildDriverAndRunTest;
pub use crate::setup::traits::birelay::BiRelaySetupComponent;
use crate::setup::traits::bootstrap_at::ProvideBootstrapAt;
use crate::setup::traits::builder_at::ProvideBuilderAt;
pub use crate::setup::traits::chain::ChainSetupComponent;
use crate::setup::traits::channel::CanSetupChannel;
pub use crate::setup::traits::channel::ChannelSetupComponent;
pub use crate::setup::traits::clients::ClientSetupComponent;
use crate::setup::traits::clients::{CanSetupClients, ClientSetup};
use crate::setup::traits::connection::CanSetupConnection;
pub use crate::setup::traits::connection::ConnectionSetupComponent;
use crate::setup::traits::create_client_options_at::{
    HasCreateClientMessageOptionsAt, HasCreateClientPayloadOptionsAt,
    ProvideCreateClientMessageOptionsAt, ProvideCreateClientPayloadOptionsAt,
};
use crate::setup::traits::driver::HasTestDriverType;
pub use crate::setup::traits::driver::{
    CanBuildTestDriver, DriverBuilderComponent, ProvideTestDriverType,
};
use crate::setup::traits::drivers::binary_channel::{
    BinaryChannelDriverBuilder, CanBuildTestDriverWithBinaryChannel,
};
use crate::setup::traits::init_channel_options_at::ProvideInitChannelOptionsAt;
use crate::setup::traits::init_connection_options_at::ProvideInitConnectionOptionsAt;
use crate::setup::traits::port_id_at::ProvidePortIdAt;
pub use crate::setup::traits::relay::RelaySetupComponent;
pub use crate::setup::traits::run_test::TestRunnerComponent;

define_components! {
    BinaryChannelTestComponents {
        DriverBuilderComponent: SetupBinaryChannelDriver,
        TestRunnerComponent: BuildDriverAndRunTest,
        ChainSetupComponent: SetupChainWithBootstrap,
        ClientSetupComponent: SetupClientsWithRelay,
        RelaySetupComponent: SetupRelayWithBuilder,
        BiRelaySetupComponent: SetupBiRelayWithBuilder,
        ConnectionSetupComponent: SetupConnectionHandshake,
        ChannelSetupComponent: SetupChannelHandshake,
    }
}

pub trait CanUseBinaryChannelTestSetup: UseBinaryChannelTestSetup {}

pub trait UseBinaryChannelTestSetup: CanBuildTestDriver {}

impl<
        Setup,
        Components,
        BootstrapA,
        BootstrapB,
        ChainA,
        ChainB,
        ChainDriverA,
        ChainDriverB,
        Relay,
        Build,
    > UseBinaryChannelTestSetup for Setup
where
    Setup: HasChainTypeAt<Index<0>, Chain = ChainA>
        + HasChainTypeAt<Index<1>, Chain = ChainB>
        + HasRelayTypeAt<Index<0>, Index<1>, Relay = Relay>
        + HasChainDriverTypeAt<Index<0>, ChainDriver = ChainDriverA>
        + HasChainDriverTypeAt<Index<1>, ChainDriver = ChainDriverB>
        + HasCreateClientPayloadOptionsAt<Index<0>, Index<1>>
        + HasCreateClientMessageOptionsAt<Index<0>, Index<1>>
        + HasCreateClientPayloadOptionsAt<Index<1>, Index<0>>
        + HasCreateClientMessageOptionsAt<Index<1>, Index<0>>
        + HasTestDriverType
        + CanRaiseError<Relay::Error>
        + CanBuildTestDriverWithBinaryChannel
        + HasComponents<Components = Components>
        + CanSetupConnection<Index<0>, Index<1>>
        + CanSetupChannel<Index<0>, Index<1>>
        // + CanSetupClients<Index<0>, Index<1>>
        + CanBuildTestDriverWithBinaryChannel
        + CanRaiseError<Relay::Error>,
    Components: DelegatesToBinaryChannelTestComponents
        + BinaryChannelDriverBuilder<Setup>
        + ProvideBootstrapAt<Setup, Index<0>, Bootstrap = BootstrapA>
        + ProvideBootstrapAt<Setup, Index<1>, Bootstrap = BootstrapB>
        + ProvideCreateClientMessageOptionsAt<Setup, Index<0>, Index<1>>
        + ProvideCreateClientMessageOptionsAt<Setup, Index<1>, Index<0>>
        + ProvideCreateClientPayloadOptionsAt<Setup, Index<0>, Index<1>>
        + ProvideCreateClientPayloadOptionsAt<Setup, Index<1>, Index<0>>
        + ProvideInitConnectionOptionsAt<Setup, Index<0>, Index<1>>
        + ProvideInitChannelOptionsAt<Setup, Index<0>, Index<1>>
        + ProvidePortIdAt<Setup, Index<0>, Index<1>>
        + ProvidePortIdAt<Setup, Index<1>, Index<0>>
        + ProvideBuilderTypeAt<Setup, Index<0>, Index<1>, Builder = Build>
        + ProvideBuilderAt<Setup, Index<0>, Index<1>>
        + ErrorRaiser<Setup, BootstrapA::Error>
        + ErrorRaiser<Setup, BootstrapB::Error>
        + ErrorRaiser<Setup, Relay::Error>
        + ErrorRaiser<Setup, Build::Error>,
    ChainDriverA: HasChain<Chain = ChainA>,
    ChainDriverB: HasChain<Chain = ChainB>,
    ChainA: HasIbcChainTypes<ChainB>
        + HasCreateClientPayloadOptionsType<ChainB>
        + HasCreateClientMessageOptionsType<ChainB>
        + HasInitConnectionOptionsType<ChainB>
        + HasInitChannelOptionsType<ChainB>
        + HasErrorType
        + Clone,
    ChainB: HasIbcChainTypes<ChainA>
        + HasCreateClientPayloadOptionsType<ChainA>
        + HasCreateClientMessageOptionsType<ChainA>
        + HasErrorType
        + Clone,
    Relay: HasRelayChains<SrcChain = ChainA, DstChain = ChainB>
        + HasTargetChains<SourceTarget>
        + HasTargetChains<DestinationTarget>
        + HasSourceTargetChainTypes
        + HasDestinationTargetChainTypes
        + HasRelayClientIds
        // + CanCreateClient<SourceTarget>
        // + CanCreateClient<DestinationTarget>
        + CanBootstrapConnection
        + CanBootstrapChannel
        + CanRaiseError<ChainA::Error>
        + CanRaiseError<ChainB::Error>,
    BootstrapA: CanBootstrapChain,
    BootstrapB: CanBootstrapChain,
    Build: HasBiRelayTypeAt<Index<0>, Index<1>, BiRelay = Setup::BiRelay>
        + HasChainTypeAt<Index<0>, Chain = ChainA>
        + HasChainTypeAt<Index<1>, Chain = ChainB>
        + HasRelayTypeAt<Index<0>, Index<1>, Relay = Relay>
        + HasRelayTypeAt<Index<1>, Index<0>, Relay = RelayAt<Setup, Index<1>, Index<0>>>
        + CanBuildRelayFromChains<Index<0>, Index<1>>
        + CanBuildRelayFromChains<Index<1>, Index<0>>
        + CanBuildBiRelayFromRelays<Index<0>, Index<1>>,
    PortIdOf<ChainA, ChainB>: Clone,
    PortIdOf<ChainB, ChainA>: Clone,
    SetupClientsWithRelay: ClientSetup<Setup, Index<0>, Index<1>>,
{
}
