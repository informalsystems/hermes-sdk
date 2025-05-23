use core::marker::PhantomData;
use std::collections::{BTreeMap, BTreeSet};

use cgp::core::component::UseContext;
use hermes_prelude::*;
use tendermint::block::Height;

pub type VerificationTrace = BTreeMap<Height, BTreeSet<Height>>;

#[cgp_component {
  provider: VerificationTraceGetter,
  context: Client,
}]
pub trait HasVerificationTrace {
    fn verification_trace(&self) -> &VerificationTrace;

    fn verification_trace_mut(&mut self) -> &mut VerificationTrace;
}

#[cgp_provider(VerificationTraceGetterComponent)]
impl<Client> VerificationTraceGetter<Client> for UseFields
where
    Client: HasFieldMut<symbol!("verification_trace"), Value = VerificationTrace>,
{
    fn verification_trace(client: &Client) -> &VerificationTrace {
        client.get_field(PhantomData)
    }

    fn verification_trace_mut(client: &mut Client) -> &mut VerificationTrace {
        client.get_field_mut(PhantomData)
    }
}
