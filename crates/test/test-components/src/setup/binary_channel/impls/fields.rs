use core::marker::PhantomData;

use cgp::core::field::Index;
use cgp::prelude::*;
use hermes_relayer_components::multi::traits::chain_at::{
    ChainTypeProviderAt, ChainTypeProviderAtComponent,
};

use crate::chain_driver::traits::types::chain::HasChainType;
use crate::driver::traits::types::chain_driver::HasChainDriverType;
use crate::driver::traits::types::chain_driver_at::{
    ChainDriverTypeProviderAt, ChainDriverTypeProviderAtComponent, HasChainDriverTypeAt,
};
use crate::setup::traits::bootstrap_at::{
    BootstrapGetterAt, BootstrapGetterAtComponent, BootstrapTypeProviderAt,
    BootstrapTypeProviderAtComponent, HasBootstrapTypeAt,
};
pub struct UseBinarySetupFields;

#[cgp_provider(BootstrapTypeProviderAtComponent<Index<0>>)]
impl<Setup, Bootstrap> BootstrapTypeProviderAt<Setup, Index<0>> for UseBinarySetupFields
where
    Setup: HasChainDriverTypeAt<Index<0>> + HasField<symbol!("bootstrap_a"), Value = Bootstrap>,
    Bootstrap: HasChainDriverType<ChainDriver = Setup::ChainDriver>,
{
    type Bootstrap = Bootstrap;
}

#[cgp_provider(BootstrapGetterAtComponent<Index<0>>)]
impl<Setup, Bootstrap> BootstrapGetterAt<Setup, Index<0>> for UseBinarySetupFields
where
    Setup: HasBootstrapTypeAt<Index<0>, Bootstrap = Bootstrap>
        + HasField<symbol!("bootstrap_a"), Value = Bootstrap>,
{
    fn chain_bootstrap(setup: &Setup, _index: PhantomData<Index<0>>) -> &Bootstrap {
        setup.get_field(PhantomData)
    }
}

#[cgp_provider(BootstrapTypeProviderAtComponent<Index<1>>)]
impl<Setup, Bootstrap> BootstrapTypeProviderAt<Setup, Index<1>> for UseBinarySetupFields
where
    Setup: HasChainDriverTypeAt<Index<1>> + HasField<symbol!("bootstrap_b"), Value = Bootstrap>,
    Bootstrap: HasChainDriverType<ChainDriver = Setup::ChainDriver>,
{
    type Bootstrap = Bootstrap;
}

#[cgp_provider(BootstrapGetterAtComponent<Index<1>>)]
impl<Setup, Bootstrap> BootstrapGetterAt<Setup, Index<1>> for UseBinarySetupFields
where
    Setup: HasBootstrapTypeAt<Index<1>, Bootstrap = Bootstrap>
        + HasField<symbol!("bootstrap_b"), Value = Bootstrap>,
{
    fn chain_bootstrap(setup: &Setup, _index: PhantomData<Index<1>>) -> &Bootstrap {
        setup.get_field(PhantomData)
    }
}

#[cgp_provider(ChainTypeProviderAtComponent<Index<0>>)]
impl<Setup, Bootstrap, Chain> ChainTypeProviderAt<Setup, Index<0>> for UseBinarySetupFields
where
    Setup: Async + HasField<symbol!("bootstrap_a"), Value = Bootstrap>,
    Bootstrap: HasChainType<Chain = Chain>,
    Chain: Async,
{
    type Chain = Chain;
}

#[cgp_provider(ChainTypeProviderAtComponent<Index<1>>)]
impl<Setup, Bootstrap, Chain> ChainTypeProviderAt<Setup, Index<1>> for UseBinarySetupFields
where
    Setup: Async + HasField<symbol!("bootstrap_b"), Value = Bootstrap>,
    Bootstrap: HasChainType<Chain = Chain>,
    Chain: Async,
{
    type Chain = Chain;
}

#[cgp_provider(ChainDriverTypeProviderAtComponent<Index<0>>)]
impl<Setup, Bootstrap, ChainDriver> ChainDriverTypeProviderAt<Setup, Index<0>>
    for UseBinarySetupFields
where
    Setup: Async + HasField<symbol!("bootstrap_a"), Value = Bootstrap>,
    Bootstrap: HasChainDriverType<ChainDriver = ChainDriver>,
    ChainDriver: Async,
{
    type ChainDriver = ChainDriver;
}

#[cgp_provider(ChainDriverTypeProviderAtComponent<Index<1>>)]
impl<Setup, Bootstrap, ChainDriver> ChainDriverTypeProviderAt<Setup, Index<1>>
    for UseBinarySetupFields
where
    Setup: Async + HasField<symbol!("bootstrap_b"), Value = Bootstrap>,
    Bootstrap: HasChainDriverType<ChainDriver = ChainDriver>,
    ChainDriver: Async,
{
    type ChainDriver = ChainDriver;
}
