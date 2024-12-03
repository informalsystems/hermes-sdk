use core::marker::PhantomData;

use cgp::prelude::*;
use hermes_relayer_components::multi::traits::chain_at::ProvideChainTypeAt;
use hermes_relayer_components::multi::types::index::Index;

use crate::chain_driver::traits::types::chain::HasChainType;
use crate::driver::traits::types::chain_driver::HasChainDriverType;
use crate::driver::traits::types::chain_driver_at::{
    HasChainDriverTypeAt, ProvideChainDriverTypeAt,
};
use crate::setup::traits::bootstrap_at::ProvideBootstrapAt;

pub struct UseBinarySetupFields;

impl<Setup, Bootstrap> ProvideBootstrapAt<Setup, Index<0>> for UseBinarySetupFields
where
    Setup: HasChainDriverTypeAt<Index<0>> + HasField<symbol!("bootstrap_a"), Field = Bootstrap>,
    Bootstrap: HasChainDriverType<ChainDriver = Setup::ChainDriver>,
{
    type Bootstrap = Bootstrap;

    fn chain_bootstrap(setup: &Setup, _index: PhantomData<Index<0>>) -> &Bootstrap {
        setup.get_field(PhantomData)
    }
}

impl<Setup, Bootstrap> ProvideBootstrapAt<Setup, Index<1>> for UseBinarySetupFields
where
    Setup: HasChainDriverTypeAt<Index<1>> + HasField<symbol!("bootstrap_b"), Field = Bootstrap>,
    Bootstrap: HasChainDriverType<ChainDriver = Setup::ChainDriver>,
{
    type Bootstrap = Bootstrap;

    fn chain_bootstrap(setup: &Setup, _index: PhantomData<Index<1>>) -> &Bootstrap {
        setup.get_field(PhantomData)
    }
}

impl<Setup, Bootstrap, Chain> ProvideChainTypeAt<Setup, Index<0>> for UseBinarySetupFields
where
    Setup: Async + HasField<symbol!("bootstrap_a"), Field = Bootstrap>,
    Bootstrap: HasChainType<Chain = Chain>,
    Chain: Async,
{
    type Chain = Chain;
}

impl<Setup, Bootstrap, Chain> ProvideChainTypeAt<Setup, Index<1>> for UseBinarySetupFields
where
    Setup: Async + HasField<symbol!("bootstrap_b"), Field = Bootstrap>,
    Bootstrap: HasChainType<Chain = Chain>,
    Chain: Async,
{
    type Chain = Chain;
}

impl<Setup, Bootstrap, ChainDriver> ProvideChainDriverTypeAt<Setup, Index<0>>
    for UseBinarySetupFields
where
    Setup: Async + HasField<symbol!("bootstrap_a"), Field = Bootstrap>,
    Bootstrap: HasChainDriverType<ChainDriver = ChainDriver>,
    ChainDriver: Async,
{
    type ChainDriver = ChainDriver;
}

impl<Setup, Bootstrap, ChainDriver> ProvideChainDriverTypeAt<Setup, Index<1>>
    for UseBinarySetupFields
where
    Setup: Async + HasField<symbol!("bootstrap_b"), Field = Bootstrap>,
    Bootstrap: HasChainDriverType<ChainDriver = ChainDriver>,
    ChainDriver: Async,
{
    type ChainDriver = ChainDriver;
}
