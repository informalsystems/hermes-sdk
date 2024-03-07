use core::fmt::Debug;
use core::marker::PhantomData;

pub struct Via<Wrapper, Value> {
    pub value: Value,
    pub phantom: PhantomData<Wrapper>,
}

impl<Wrapper, Value> AsRef<Value> for Via<Wrapper, Value> {
    fn as_ref(&self) -> &Value {
        &self.value
    }
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

impl<Wrapper, Value> Debug for Via<Wrapper, Value>
where
    Value: Debug,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.value.fmt(f)
    }
}
