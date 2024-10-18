use core::marker::PhantomData;

pub struct WithAppProvider<Provider>(pub PhantomData<Provider>);
