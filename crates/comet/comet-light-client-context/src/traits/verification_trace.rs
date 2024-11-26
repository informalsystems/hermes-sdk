use core::marker::PhantomData;
use std::collections::{BTreeMap, BTreeSet};

use cgp::core::component::UseContext;
use cgp::prelude::*;
use tendermint::block::Height;

pub type VerificationTrace = BTreeMap<Height, BTreeSet<Height>>;

#[derive_component(VerificationTraceGetterComponent, VerificationTraceGetter<Client>)]
pub trait HasVerificationTrace {
    fn verification_trace(&self) -> &VerificationTrace;

    fn verification_trace_mut(&mut self) -> &mut VerificationTrace;
}

impl<Client> VerificationTraceGetter<Client> for UseContext
where
    Client: HasFieldMut<symbol!("verification_trace"), Field = VerificationTrace>,
{
    fn verification_trace(client: &Client) -> &VerificationTrace {
        client.get_field(PhantomData)
    }

    fn verification_trace_mut(client: &mut Client) -> &mut VerificationTrace {
        client.get_field_mut(PhantomData)
    }
}
