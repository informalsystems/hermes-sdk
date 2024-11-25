use std::collections::{BTreeMap, BTreeSet};

use cgp::prelude::*;
use tendermint::block::Height;

#[derive_component(VerificationTraceGetterComponent, VerificationTraceGetter<Client>)]
pub trait HasVerificationTrace {
    fn verification_trace_mut(&mut self) -> &mut BTreeMap<Height, BTreeSet<Height>>;
}
