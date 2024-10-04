use core::marker::PhantomData;
use core::ops::Deref;

pub struct Tagged<Chain, Counterparty, Value> {
    pub value: Value,
    pub phantom: PhantomData<(Chain, Counterparty)>,
}

impl<Chain, Counterparty, Value> Deref for Tagged<Chain, Counterparty, Value> {
    type Target = Value;

    fn deref(&self) -> &Value {
        &self.value
    }
}
