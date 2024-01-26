use cgp_core::HasInner;

use crate::chain::traits::types::channel::{
    HasInitChannelOptionsType, ProvideInitChannelOptionsType,
};

pub struct ForwardInitChannelOptionsType;

impl<Chain, Counterparty, Inner> ProvideInitChannelOptionsType<Chain, Counterparty>
    for ForwardInitChannelOptionsType
where
    Chain: HasInner<Inner = Inner>,
    Inner: HasInitChannelOptionsType<Counterparty>,
{
    type InitChannelOptions = Inner::InitChannelOptions;
}
