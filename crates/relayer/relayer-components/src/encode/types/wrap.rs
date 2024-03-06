use core::marker::PhantomData;

pub struct Wrap<Wrapper, Value> {
    pub value: Value,
    pub phantom: PhantomData<Wrapper>,
}

impl<Wrapper, Value> From<Value> for Wrap<Wrapper, Value> {
    fn from(value: Value) -> Self {
        Wrap {
            value,
            phantom: PhantomData,
        }
    }
}

impl<Wrapper, Value> Default for Wrap<Wrapper, Value>
where
    Value: Default,
{
    fn default() -> Self {
        Value::default().into()
    }
}
