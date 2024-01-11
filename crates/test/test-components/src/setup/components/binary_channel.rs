use cgp_core::prelude::*;
use cgp_core::ErrorRaiser;
use cgp_core::ProvideErrorType;
use hermes_relayer_components::birelay::traits::two_way::HasTwoWayRelay;
use hermes_relayer_components::build::traits::birelay::HasBiRelayType;
use hermes_relayer_components::build::traits::components::birelay_from_relay_builder::CanBuildBiRelayFromRelays;
use hermes_relayer_components::build::traits::components::relay_from_chains_builder::CanBuildRelayFromChains;
use hermes_relayer_components::build::traits::target::relay::RelayAToBTarget;
use hermes_relayer_components::build::traits::target::relay::RelayBToATarget;
use hermes_relayer_components::chain::traits::types::create_client::HasCreateClientOptionsType;
use hermes_relayer_components::chain::traits::types::ibc::HasIbcChainTypes;
use hermes_relayer_components::relay::traits::chains::HasRelayChains;
use hermes_relayer_components::relay::traits::components::client_creator::CanCreateClient;
use hermes_relayer_components::relay::traits::target::DestinationTarget;
use hermes_relayer_components::relay::traits::target::SourceTarget;

use crate::bootstrap::traits::chain::CanBootstrapChain;
use crate::driver::traits::types::birelay_at::ProvideBiRelayTypeAt;
use crate::driver::traits::types::builder_at::ProvideBuilderTypeAt;
use crate::driver::traits::types::chain_at::ProvideChainTypeAt;
use crate::driver::traits::types::relay_at::ProvideRelayTypeAt;
use crate::setup::impls::binary_channel::SetupBinaryChannelDriver;
use crate::setup::impls::birelay::SetupBiRelayWithBuilder;
use crate::setup::impls::chain::SetupChainWithBootstrap;
use crate::setup::impls::channel::SetupChannelHandshake;
use crate::setup::impls::clients::SetupClientsWithRelay;
use crate::setup::impls::connection::SetupConnectionHandshake;
use crate::setup::impls::relay::SetupRelayWithBuilder;
use crate::setup::impls::run_test::BuildDriverAndRunTest;
use crate::setup::traits::birelay::{BiRelaySetup, BiRelaySetupComponent};
use crate::setup::traits::bootstrap_at::ProvideBootstrapAt;
use crate::setup::traits::builder_at::ProvideBuilderAt;
use crate::setup::traits::chain::ChainSetupComponent;
use crate::setup::traits::channel::{ChannelSetup, ChannelSetupComponent};
use crate::setup::traits::clients::ClientSetupComponent;
use crate::setup::traits::connection::{ConnectionSetup, ConnectionSetupComponent};
use crate::setup::traits::create_client_options_at::ProvideCreateClientOptionsAt;
use crate::setup::traits::driver::{CanBuildDriver, DriverBuilderComponent, ProvideDriverType};
use crate::setup::traits::drivers::binary_channel::BinaryChannelDriverBuilder;
use crate::setup::traits::relay::RelaySetupComponent;
use crate::setup::traits::run_test::TestRunnerComponent;

pub struct BinaryChannelTestComponents;

delegate_components! {
    #[mark_component(IsBinaryChannelTestComponent)]
    #[mark_delegate(DelegatesToBinaryChannelTestComponents)]
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

pub trait UseBinaryChannelTestSetup: CanBuildDriver {}

impl<Setup, Components, ChainA, ChainB, BootstrapA, BootstrapB, Relay, BiRelay, Build>
    UseBinaryChannelTestSetup for Setup
where
    Setup: HasComponents<Components = Components>,
    Components: DelegatesToBinaryChannelTestComponents
        + ProvideDriverType<Setup>
        + ProvideErrorType<Setup>
        + ProvideChainTypeAt<Setup, 0, Chain = ChainA>
        + ProvideChainTypeAt<Setup, 1, Chain = ChainB>
        + ProvideRelayTypeAt<Setup, 0, 1, Relay = Relay>
        + ProvideRelayTypeAt<Setup, 1, 0>
        + ProvideBiRelayTypeAt<Setup, 0, 1, BiRelay = BiRelay>
        + BinaryChannelDriverBuilder<Setup>
        + ProvideBootstrapAt<Setup, 0, Bootstrap = BootstrapA>
        + ProvideBootstrapAt<Setup, 1, Bootstrap = BootstrapB>
        + ProvideCreateClientOptionsAt<Setup, 0, 1>
        + ProvideCreateClientOptionsAt<Setup, 1, 0>
        + ProvideBuilderTypeAt<Setup, 0, 1, Builder = Build>
        + ProvideBuilderAt<Setup, 0, 1>
        + ErrorRaiser<Setup, BootstrapA::Error>
        + ErrorRaiser<Setup, BootstrapB::Error>
        + ErrorRaiser<Setup, Relay::Error>
        + ErrorRaiser<Setup, Build::Error>,
    SetupBiRelayWithBuilder: BiRelaySetup<Setup, 0, 1>,
    SetupConnectionHandshake: ConnectionSetup<Setup, 0, 1>,
    SetupChannelHandshake: ChannelSetup<Setup, 0, 1>,
    ChainA: HasIbcChainTypes<ChainB> + HasCreateClientOptionsType<ChainB> + HasErrorType + Clone,
    ChainB: HasIbcChainTypes<ChainA> + HasCreateClientOptionsType<ChainA> + HasErrorType + Clone,
    Relay: HasRelayChains<SrcChain = ChainA, DstChain = ChainB>
        + CanCreateClient<SourceTarget>
        + CanCreateClient<DestinationTarget>,
    BootstrapA: CanBootstrapChain,
    BootstrapB: CanBootstrapChain,
    Build: HasBiRelayType<BiRelay = BiRelay>
        + CanBuildRelayFromChains<RelayAToBTarget>
        + CanBuildRelayFromChains<RelayBToATarget>
        + CanBuildBiRelayFromRelays,
    BiRelay: HasTwoWayRelay<ChainA = ChainA, ChainB = ChainB>,
{
}
