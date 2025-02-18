#[cgp::re_export_imports]
mod preset {
    use cgp::prelude::*;

    use crate::setup::binary_channel::impls::setup::SetupBinaryChannelDriver;
    use crate::setup::impls::birelay::SetupBiRelayWithBuilder;
    use crate::setup::impls::chain::SetupChainWithBootstrap;
    use crate::setup::impls::channel::SetupChannelHandshake;
    use crate::setup::impls::clients::SetupClientsWithRelay;
    use crate::setup::impls::connection::SetupConnectionHandshake;
    use crate::setup::impls::relay::SetupRelayWithBuilder;
    use crate::setup::impls::run_test::BuildDriverAndRunTest;
    use crate::setup::traits::birelay::BiRelaySetupComponent;
    use crate::setup::traits::chain::ChainSetupComponent;
    use crate::setup::traits::channel::ChannelSetupComponent;
    use crate::setup::traits::clients::ClientSetupComponent;
    use crate::setup::traits::connection::ConnectionSetupComponent;
    use crate::setup::traits::driver::{
        CanBuildTestDriver, DriverBuilderComponent, ProvideTestDriverType,
    };
    use crate::setup::traits::relay::RelaySetupComponent;
    use crate::setup::traits::run_test::TestRunnerComponent;

    cgp_preset! {
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
}
