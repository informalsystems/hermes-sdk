use core::fmt::Display;

use cgp::core::component::WithProvider;
use cgp::core::types::traits::ProvideType;
use cgp::prelude::*;

#[derive_component(HeightTypeComponent, ProvideHeightType<Chain>)]
pub trait HasHeightType: Async {
    /**
       The height of the chain, which should behave like natural numbers.

       By default, the height only contains the `Ord` constraint, and does
       not support operations like addition.

       We can impose additional constraints at the use site of `HasChainTypes`.
       However doing so may impose limitations on which concrete types
       the `Height` type can be.

       By keeping the abstract type minimal, we can for example use
       `u8` or `u128` as the `Height` type during testing, and use the
       more complex Cosmos height type during production.
    */
    type Height: Ord + Display + Async + Clone;
}

impl<Chain, Provider, Height> ProvideHeightType<Chain> for WithProvider<Provider>
where
    Chain: Async,
    Provider: ProvideType<Chain, HeightTypeComponent, Type = Height>,
    Height: Ord + Display + Async + Clone,
{
    type Height = Height;
}
