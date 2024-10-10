use core::fmt::{Debug, Display};
use core::marker::PhantomData;
use core::ops::Deref;

pub struct Tagged<Chain, Counterparty, Value> {
    pub value: Value,
    pub phantom: PhantomData<(Chain, Counterparty)>,
}

impl<A, B, Value> From<Value> for Tagged<A, B, Value> {
    fn from(value: Value) -> Self {
        Self {
            value,
            phantom: PhantomData,
        }
    }
}

impl<A, B, Value> Deref for Tagged<A, B, Value> {
    type Target = Value;

    fn deref(&self) -> &Value {
        &self.value
    }
}

impl<A, B, Value> Debug for Tagged<A, B, Value>
where
    Value: Debug,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.value.fmt(f)
    }
}

impl<A, B, Value> Display for Tagged<A, B, Value>
where
    Value: Display,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.value.fmt(f)
    }
}

impl<A, B, Value> Clone for Tagged<A, B, Value>
where
    Value: Clone,
{
    fn clone(&self) -> Self {
        self.value.clone().into()
    }
}
