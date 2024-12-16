use core::marker::PhantomData;

use cgp::core::component::UseContext;
use cgp::prelude::*;
use tendermint_rpc::client::CompatMode;

#[cgp_component {
  provider: CompatModeGetter,
  context: Bootstrap,
}]
pub trait HasCompatMode: Async {
    fn compat_mode(&self) -> Option<&CompatMode>;
}

impl<Bootstrap> CompatModeGetter<Bootstrap> for UseContext
where
    Bootstrap: Async + HasField<symbol!("compat_mode"), Value = Option<CompatMode>>,
{
    fn compat_mode(bootstrap: &Bootstrap) -> Option<&CompatMode> {
        bootstrap.get_field(PhantomData).as_ref()
    }
}

pub struct UseCompatMode34;
pub struct UseCompatMode37;

impl<Bootstrap: Async> CompatModeGetter<Bootstrap> for UseCompatMode34 {
    fn compat_mode(_bootstrap: &Bootstrap) -> Option<&CompatMode> {
        const COMPAT_MODE: CompatMode = CompatMode::V0_34;
        Some(&COMPAT_MODE)
    }
}

impl<Bootstrap: Async> CompatModeGetter<Bootstrap> for UseCompatMode37 {
    fn compat_mode(_bootstrap: &Bootstrap) -> Option<&CompatMode> {
        const COMPAT_MODE: CompatMode = CompatMode::V0_37;
        Some(&COMPAT_MODE)
    }
}
