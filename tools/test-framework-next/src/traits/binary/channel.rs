use ibc_relayer_components::chain::traits::types::ibc::HasIbcChainTypes;
use ibc_relayer_components::chain::types::aliases::{ChannelId, PortId};
use ibc_relayer_components::relay::traits::two_way::HasTwoChainTypes;

pub trait HasTwoChannels: HasTwoChainTypes
where
    Self::ChainA: HasIbcChainTypes<Self::ChainB>,
    Self::ChainB: HasIbcChainTypes<Self::ChainA>,
{
    fn channel_id_a(&self) -> &ChannelId<Self::ChainA, Self::ChainB>;

    fn channel_id_b(&self) -> &ChannelId<Self::ChainB, Self::ChainA>;

    fn port_id_a(&self) -> &PortId<Self::ChainA, Self::ChainB>;

    fn port_id_b(&self) -> &PortId<Self::ChainB, Self::ChainA>;
}
