use crate::framework::binary::next::TestContextV1;
use crate::framework::next::chain::{
    CanShutdown, CanSpawnRelayer, CanWaitForAck, HasContextId, HasTestConfig, HasTwoChannels,
    HasTwoNodes,
};
use crate::prelude::*;

pub fn build_test_context<ChainA: ChainHandle, ChainB: ChainHandle>(
    config: &TestConfig,
    relayer: RelayerDriver,
    chains: ConnectedChains<ChainA, ChainB>,
    channels: ConnectedChannel<ChainA, ChainB>,
) -> Result<
    impl HasTwoChannels
        + HasTwoNodes
        + HasTestConfig
        + CanSpawnRelayer
        + HasContextId
        + CanWaitForAck
        + CanShutdown,
    Error,
> {
    let context_current = TestContextV1 {
        context_id: "current_relayer".to_owned(),
        config: config.clone(),
        relayer,
        chains,
        channel: channels,
    };

    Ok(context_current)
}
