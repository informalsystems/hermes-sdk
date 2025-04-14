#[cgp::re_export_imports]
mod preset {
    use cgp::prelude::*;

    use crate::setup::binary_channel::SetupBinaryChannelDriver;
    use crate::setup::impls::{
        BuildDriverAndRunTest, SetupBiRelayWithBuilder, SetupChainWithBootstrap,
        SetupChannelHandshake, SetupClientsWithRelay, SetupConnectionHandshake,
        SetupRelayWithBuilder,
    };
    use crate::setup::traits::{
        BiRelaySetupComponent, ChainSetupComponent, ChannelSetupComponent, ClientSetupComponent,
        ConnectionSetupComponent, DriverBuilderComponent, RelaySetupComponent, TestRunnerComponent,
    };

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
