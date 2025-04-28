use core::fmt::{Debug, Display};

use hermes_prelude::*;

#[cgp_component {
  name: SequenceTypeComponent,
  provider: ProvideSequenceType,
  context: Chain,
}]
pub trait HasSequenceType<Counterparty>: Async {
    /**
       The IBC packet sequence for the packet that is sent from the self chain
       to the counterparty chain.

       Note that for sequences of packets that are sent from the counterparty
       chain to self, the `Counterparty::Sequence` will be used.
    */
    type Sequence: Debug + Display + Async;
}
