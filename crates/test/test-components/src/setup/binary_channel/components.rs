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
use hermes_relayer_components::multi::traits::chain_at::{ChainAt, HasChainTypeAt};
use hermes_relayer_components::multi::traits::relay_at::{HasRelayTypeAt, RelayAt};
use hermes_relayer_components::relay::impls::channel::bootstrap::CanBootstrapChannel;
use hermes_relayer_components::relay::impls::connection::bootstrap::CanBootstrapConnection;
use hermes_relayer_components::relay::traits::chains::{CanRaiseRelayChainErrors, HasRelayChains};
use hermes_relayer_components::relay::traits::client_creator::CanCreateClient;
use hermes_relayer_components::relay::traits::target::{DestinationTarget, SourceTarget};

use crate::bootstrap::traits::chain::CanBootstrapChain;
use crate::chain_driver::traits::types::chain::HasChain;
use crate::driver::traits::types::builder_at::ProvideBuilderTypeAt;
use crate::driver::traits::types::chain_driver_at::{ChainDriverTypeAt, HasChainDriverTypeAt};
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
use crate::setup::traits::connection::CanSetupConnection;
pub use crate::setup::traits::connection::ConnectionSetupComponent;
use crate::setup::traits::create_client_options_at::ProvideCreateClientOptionsAt;
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

impl<Setup, Components, BootstrapA, BootstrapB, Relay, Build> UseBinaryChannelTestSetup for Setup
where
    Setup: HasChainTypeAt<0>
        + HasChainTypeAt<1>
        + HasRelayTypeAt<0, 1, Relay = Relay>
        + HasRelayTypeAt<1, 0>
        + HasBiRelayTypeAt<0, 1>
        + HasChainDriverTypeAt<0>
        + HasChainDriverTypeAt<1>
        + HasTestDriverType
        + HasErrorType
        + HasComponents<Components = Components>
        + CanSetupConnection<0, 1>
        + CanSetupChannel<0, 1>
        + CanBuildTestDriverWithBinaryChannel,
    Components: DelegatesToBinaryChannelTestComponents
        + BinaryChannelDriverBuilder<Setup>
        + ProvideBootstrapAt<Setup, 0, Bootstrap = BootstrapA>
        + ProvideBootstrapAt<Setup, 1, Bootstrap = BootstrapB>
        + ProvideCreateClientOptionsAt<Setup, 0, 1>
        + ProvideCreateClientOptionsAt<Setup, 1, 0>
        + ProvideInitConnectionOptionsAt<Setup, 0, 1>
        + ProvideInitChannelOptionsAt<Setup, 0, 1>
        + ProvidePortIdAt<Setup, 0, 1>
        + ProvidePortIdAt<Setup, 1, 0>
        + ProvideBuilderTypeAt<Setup, 0, 1, Builder = Build>
        + ProvideBuilderAt<Setup, 0, 1>
        + ErrorRaiser<Setup, BootstrapA::Error>
        + ErrorRaiser<Setup, BootstrapB::Error>
        + ErrorRaiser<Setup, Relay::Error>
        + ErrorRaiser<Setup, Build::Error>,
    ChainDriverTypeAt<Setup, 0>: HasChain<Chain = ChainAt<Setup, 0>>,
    ChainDriverTypeAt<Setup, 1>: HasChain<Chain = ChainAt<Setup, 1>>,
    ChainAt<Setup, 0>: HasIbcChainTypes<ChainAt<Setup, 1>>
        + HasCreateClientPayloadOptionsType<ChainAt<Setup, 1>>
        + HasCreateClientMessageOptionsType<ChainAt<Setup, 1>>
        + HasInitConnectionOptionsType<ChainAt<Setup, 1>>
        + HasInitChannelOptionsType<ChainAt<Setup, 1>>
        + HasErrorType
        + Clone,
    ChainAt<Setup, 1>: HasIbcChainTypes<ChainAt<Setup, 0>>
        + HasCreateClientPayloadOptionsType<ChainAt<Setup, 0>>
        + HasCreateClientMessageOptionsType<ChainAt<Setup, 0>>
        + HasErrorType
        + Clone,
    Relay: HasRelayChains<SrcChain = ChainAt<Setup, 0>, DstChain = ChainAt<Setup, 1>>
        + CanCreateClient<SourceTarget>
        + CanCreateClient<DestinationTarget>
        + CanBootstrapConnection
        + CanBootstrapChannel
        + CanRaiseRelayChainErrors,
    BootstrapA: CanBootstrapChain,
    BootstrapB: CanBootstrapChain,
    Build: HasBiRelayTypeAt<0, 1, BiRelay = Setup::BiRelay>
        + HasChainTypeAt<0, Chain = ChainAt<Setup, 0>>
        + HasChainTypeAt<1, Chain = ChainAt<Setup, 1>>
        + HasRelayTypeAt<0, 1, Relay = Relay>
        + HasRelayTypeAt<1, 0, Relay = RelayAt<Setup, 1, 0>>
        + CanBuildRelayFromChains<0, 1>
        + CanBuildRelayFromChains<1, 0>
        + CanBuildBiRelayFromRelays<0, 1>,
    PortIdOf<ChainAt<Setup, 0>, ChainAt<Setup, 1>>: Clone,
    PortIdOf<ChainAt<Setup, 1>, ChainAt<Setup, 0>>: Clone,
{
}
