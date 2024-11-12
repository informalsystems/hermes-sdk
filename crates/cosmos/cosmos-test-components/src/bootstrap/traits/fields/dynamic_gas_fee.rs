use core::marker::PhantomData;

use cgp::core::component::UseContext;
use cgp::prelude::*;
use ibc_relayer::config::dynamic_gas::DynamicGasPrice;

#[derive_component(DynamicGasGetterComponent, DynamicGasGetter<Bootstrap>)]
pub trait HasDynamicGas: Async {
    fn dynamic_gas(&self) -> &DynamicGasPrice;
}

impl<Bootstrap> DynamicGasGetter<Bootstrap> for UseContext
where
    Bootstrap: Async + HasField<symbol!("dynamic_gas"), Field = DynamicGasPrice>,
{
    fn dynamic_gas(bootstrap: &Bootstrap) -> &DynamicGasPrice {
        bootstrap.get_field(PhantomData)
    }
}
