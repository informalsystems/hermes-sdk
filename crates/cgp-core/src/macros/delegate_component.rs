#[macro_export]
macro_rules! delegate_component {
    ( $key:ty, $target:ident $( < $( $param:ident ),* $(,)? > )?, $forwarded:ty $(,)?  ) => {
        impl< $( $( $param ),* )* >
            $crate::traits::delegate_component::DelegateComponent<$key>
            for $target $( < $( $param ),* > )*
        where
            Self: $crate::traits::sync::Async,
        {
            type Delegate = $forwarded;
        }
    };
}
