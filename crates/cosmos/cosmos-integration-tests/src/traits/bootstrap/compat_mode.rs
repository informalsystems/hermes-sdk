use hermes_prelude::*;
use tendermint_rpc::client::CompatMode;

#[cgp_getter {
    provider: CompatModeGetter,
}]
pub trait HasCompatMode: Async {
    fn compat_mode(&self) -> Option<&CompatMode>;
}

#[cgp_new_provider(CompatModeGetterComponent)]
impl<Bootstrap: Async> CompatModeGetter<Bootstrap> for UseCompatMode34 {
    fn compat_mode(_bootstrap: &Bootstrap) -> Option<&CompatMode> {
        const COMPAT_MODE: CompatMode = CompatMode::V0_34;
        Some(&COMPAT_MODE)
    }
}

#[cgp_new_provider(CompatModeGetterComponent)]
impl<Bootstrap: Async> CompatModeGetter<Bootstrap> for UseCompatMode37 {
    fn compat_mode(_bootstrap: &Bootstrap) -> Option<&CompatMode> {
        const COMPAT_MODE: CompatMode = CompatMode::V0_37;
        Some(&COMPAT_MODE)
    }
}
