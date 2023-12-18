use crate::runtime::traits::mutex::HasMutex;
use crate::runtime::traits::runtime::HasRuntimeType;

pub type Runtime<Context> = <Context as HasRuntimeType>::Runtime;

pub type Mutex<Context, T> = <Runtime<Context> as HasMutex>::Mutex<T>;
