use cgp::prelude::*;
use ibc_relayer::config::compat_mode::CompatMode;

#[derive_component(CompatModeGetterComponent, CompatModeGetter<Bootstrap>)]
pub trait HasCompatMode: Async {
    fn compat_mode(&self) -> Option<&CompatMode>;
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
