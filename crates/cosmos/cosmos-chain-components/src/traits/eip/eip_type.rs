use cgp::core::component::UseContext;
use cgp::prelude::*;
use core::marker::PhantomData;

#[derive(Clone, Default)]
pub enum EipQueryType {
    FeeMarket,
    #[default]
    Osmosis,
}

#[derive_component(EipQueryTypeGetterComponent, EipQueryTypeGetter<Chain>)]
pub trait HasEipQueryType: Async {
    fn eip_query_type(&self) -> &EipQueryType;
}

impl<Chain> EipQueryTypeGetter<Chain> for UseContext
where
    Chain: Async + HasField<symbol!("eip_query_type"), Field = EipQueryType>,
{
    fn eip_query_type(chain: &Chain) -> &EipQueryType {
        chain.get_field(PhantomData)
    }
}
