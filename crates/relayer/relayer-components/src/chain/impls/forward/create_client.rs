use cgp_core::HasInner;

use crate::chain::traits::types::create_client::{
    HasCreateClientOptionsType, ProvideCreateClientOptionsType,
};

pub struct ForwardCreateClientOptionsType;

impl<Chain, Counterparty, Inner> ProvideCreateClientOptionsType<Chain, Counterparty>
    for ForwardCreateClientOptionsType
where
    Chain: HasInner<Inner = Inner>,
    Inner: HasCreateClientOptionsType<Counterparty>,
{
    type CreateClientOptions = Inner::CreateClientOptions;
}
