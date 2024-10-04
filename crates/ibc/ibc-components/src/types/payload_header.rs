use core::marker::PhantomData;

pub struct IbcPayloadHeader<Chain, Counterparty> {
    pub phantom: PhantomData<(Chain, Counterparty)>,
}
