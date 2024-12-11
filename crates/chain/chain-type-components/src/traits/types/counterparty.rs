pub trait HasCounterparty<Counterparty> {
    type Counterparty;
}

impl<Chain, Counterparty> HasCounterparty<Counterparty> for Chain {
    type Counterparty = Counterparty;
}

pub trait CanUseCounterparty<Counterparty>:
    HasCounterparty<Counterparty, Counterparty = Counterparty>
{
}

impl<Chain, Counterparty> CanUseCounterparty<Counterparty> for Chain {}
