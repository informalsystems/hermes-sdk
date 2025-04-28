use core::fmt::{Debug, Display};

use cgp::core::component::WithProvider;
use cgp::core::types::ProvideType;
use hermes_prelude::*;

#[cgp_type {
    name: AmountTypeProviderComponent,
    provider: AmountTypeProvider,
    context: Chain,
}]
pub trait HasAmountType: Async {
    type Amount: Debug + Display + Eq + Ord + Clone + Async;
}
