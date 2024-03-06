use core::marker::PhantomData;

pub struct Via<Wrapper, Value> {
    pub value: Value,
    pub phantom: PhantomData<Wrapper>,
}

impl<Wrapper, Value> From<Value> for Via<Wrapper, Value> {
    fn from(value: Value) -> Self {
        Via {
            value,
            phantom: PhantomData,
        }
    }
}

impl<Wrapper, Value> Default for Via<Wrapper, Value>
where
    Value: Default,
{
    fn default() -> Self {
        Value::default().into()
    }
}
