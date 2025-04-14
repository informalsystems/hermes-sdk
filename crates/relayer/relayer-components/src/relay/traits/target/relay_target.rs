use cgp::core::Async;

use crate::multi::types::tags::{Dst, Src};
use crate::relay::traits::{DestinationTarget, SourceTarget};

pub trait RelayTarget: Async + Default + Copy + private::Sealed {
    type Chain: Async;

    type Counterparty: Async;
}

impl RelayTarget for SourceTarget {
    type Chain = Src;

    type Counterparty = Dst;
}

impl RelayTarget for DestinationTarget {
    type Chain = Dst;

    type Counterparty = Src;
}

impl private::Sealed for SourceTarget {}
impl private::Sealed for DestinationTarget {}

mod private {
    pub trait Sealed {}
}
