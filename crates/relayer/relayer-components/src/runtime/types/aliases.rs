use crate::runtime::traits::mutex::HasMutex;
use crate::runtime::traits::runtime::HasRuntimeType;

pub type RuntimeOf<Context> = <Context as HasRuntimeType>::Runtime;

pub type Mutex<Context, T> = <RuntimeOf<Context> as HasMutex>::Mutex<T>;
