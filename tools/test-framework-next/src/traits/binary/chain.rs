use ibc_relayer_components::relay::traits::two_way::HasTwoChainTypes;

pub trait HasTwoChains: HasTwoChainTypes {
    fn chain_a(&self) -> &Self::ChainA;

    fn chain_b(&self) -> &Self::ChainB;
}
