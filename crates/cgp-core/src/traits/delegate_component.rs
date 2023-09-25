use crate::traits::sync::Async;

pub trait DelegateComponent<Name>: Async {
    type Delegate;
}
