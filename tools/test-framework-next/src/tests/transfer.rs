use async_trait::async_trait;
use cgp_core::{HasErrorType, Runner};
use ibc_relayer_components::build::traits::birelay::HasBiRelayType;
use ibc_relayer_components::relay::traits::two_way::HasTwoWayRelay;

pub struct TestIbcTransfer;

#[async_trait]
impl<Test, ChainA, ChainB> Runner<Test> for TestIbcTransfer
where
    Test: HasErrorType + HasBiRelayType,
    Test::BiRelay: HasTwoWayRelay<ChainA = ChainA, ChainB = ChainB>,
{
    async fn run(_test: &Test) -> Result<(), Test::Error> {
        Ok(())
    }
}
