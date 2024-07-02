use cgp_core::error::{ErrorRaiser, ProvideErrorType};
use cgp_core::prelude::*;
use hermes_relayer_components::birelay::traits::two_way::HasTwoWayRelay;
use hermes_relayer_components::build::traits::birelay::HasBiRelayType;
use hermes_relayer_components::build::traits::components::birelay_from_relay_builder::CanBuildBiRelayFromRelays;
use hermes_relayer_components::build::traits::components::relay_from_chains_builder::CanBuildRelayFromChains;
use hermes_relayer_components::build::traits::target::relay::{RelayAToBTarget, RelayBToATarget};
use hermes_relayer_components::chain::traits::types::channel::HasInitChannelOptionsType;
use hermes_relayer_components::chain::traits::types::connection::HasInitConnectionOptionsType;
use hermes_relayer_components::chain::traits::types::create_client::{
    HasCreateClientMessageOptionsType, HasCreateClientPayloadOptionsType,
};
use hermes_relayer_components::chain::traits::types::ibc::HasIbcChainTypes;
use hermes_relayer_components::relay::impls::channel::bootstrap::CanBootstrapChannel;
use hermes_relayer_components::relay::impls::connection::bootstrap::CanBootstrapConnection;
use hermes_relayer_components::relay::traits::chains::{CanRaiseRelayChainErrors, HasRelayChains};
use hermes_relayer_components::relay::traits::client_creator::CanCreateClient;
use hermes_relayer_components::relay::traits::target::{DestinationTarget, SourceTarget};

use crate::bootstrap::traits::chain::CanBootstrapChain;
use crate::chain_driver::traits::types::chain::HasChain;
use crate::driver::traits::types::birelay_at::ProvideBiRelayTypeAt;
use crate::driver::traits::types::builder_at::ProvideBuilderTypeAt;
use crate::driver::traits::types::chain_at::ProvideChainTypeAt;
use crate::driver::traits::types::chain_driver_at::ProvideChainDriverTypeAt;
use crate::driver::traits::types::relay_at::ProvideRelayTypeAt;
use crate::setup::binary_channel::setup::SetupBinaryChannelDriver;
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
pub use crate::setup::traits::channel::ChannelSetupComponent;
pub use crate::setup::traits::clients::ClientSetupComponent;
pub use crate::setup::traits::connection::ConnectionSetupComponent;
use crate::setup::traits::create_client_options_at::ProvideCreateClientOptionsAt;
pub use crate::setup::traits::driver::{
    CanBuildTestDriver, DriverBuilderComponent, ProvideTestDriverType,
};
use crate::setup::traits::drivers::binary_channel::BinaryChannelDriverBuilder;
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
        ChainDriverA,
        ChainDriverB,
        ChainA,
        ChainB,
        BootstrapA,
        BootstrapB,
        Relay,
        BiRelay,
        Build,
    > UseBinaryChannelTestSetup for Setup
where
    Setup: HasComponents<Components = Components>,
    Components: DelegatesToBinaryChannelTestComponents
        + ProvideTestDriverType<Setup>
        + ProvideErrorType<Setup>
        + ProvideChainTypeAt<Setup, 0, Chain = ChainA>
        + ProvideChainTypeAt<Setup, 1, Chain = ChainB>
        + ProvideChainDriverTypeAt<Setup, 0, ChainDriver = ChainDriverA>
        + ProvideChainDriverTypeAt<Setup, 1, ChainDriver = ChainDriverB>
        + ProvideRelayTypeAt<Setup, 0, 1, Relay = Relay>
        + ProvideRelayTypeAt<Setup, 1, 0>
        + ProvideBiRelayTypeAt<Setup, 0, 1, BiRelay = BiRelay>
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
        + CanCreateClient<SourceTarget>
        + CanCreateClient<DestinationTarget>
        + CanBootstrapConnection
        + CanBootstrapChannel
        + CanRaiseRelayChainErrors,
    BootstrapA: CanBootstrapChain,
    BootstrapB: CanBootstrapChain,
    Build: HasBiRelayType<BiRelay = BiRelay>
        + CanBuildRelayFromChains<RelayAToBTarget>
        + CanBuildRelayFromChains<RelayBToATarget>
        + CanBuildBiRelayFromRelays,
    BiRelay: HasTwoWayRelay<ChainA = ChainA, ChainB = ChainB>,
    ChainA::PortId: Clone,
    ChainB::PortId: Clone,
{
}
