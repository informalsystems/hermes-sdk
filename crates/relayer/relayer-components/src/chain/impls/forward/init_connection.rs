use cgp_core::HasInner;

use crate::chain::traits::types::connection::{
    HasInitConnectionOptionsType, ProvideInitConnectionOptionsType,
};

pub struct ForwardInitConnectionOptionsType;

impl<Chain, Counterparty, Inner> ProvideInitConnectionOptionsType<Chain, Counterparty>
    for ForwardInitConnectionOptionsType
where
    Chain: HasInner<Inner = Inner>,
    Inner: HasInitConnectionOptionsType<Counterparty>,
{
    type InitConnectionOptions = Inner::InitConnectionOptions;
}
