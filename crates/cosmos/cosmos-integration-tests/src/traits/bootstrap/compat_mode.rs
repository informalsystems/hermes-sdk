use cgp::prelude::*;
use ibc_relayer::config::compat_mode::CompatMode;

#[derive_component(CompatModeGetterComponent, CompatModeGetter<Bootstrap>)]
pub trait HasCompatMode: Async {
    fn compat_mode(&self) -> Option<&CompatMode>;
}
