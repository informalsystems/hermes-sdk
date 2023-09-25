#[macro_export]
macro_rules! delegate_components {
    ( [$(,)?], $target:ident $( < $( $param:ident ),* $(,)? > )?, $forwarded:ty $(,)? ) => {

    };
    ( [$name:ty $(, $($rest:tt)* )?], $target:ident $( < $( $param:ident ),* $(,)? > )?, $forwarded:ty $(,)?  ) => {
        $crate::delegate_component!($name, $target $( < $( $param ),* > )*, $forwarded);
        $crate::delegate_components!([ $( $($rest)* )? ], $target $( < $( $param ),* > )*, $forwarded);
    };
}
