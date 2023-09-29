#![no_std]
#![allow(clippy::type_complexity)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::needless_lifetimes)]

mod std_prelude;
extern crate alloc;

pub mod build;
pub mod chain;
pub mod components;
pub mod core;
pub mod logger;
pub mod relay;
pub mod runtime;
pub mod transaction;

#[macro_export]
macro_rules! strip_async {
    (   @ins( )
        @out( $( $out:tt )* )
    ) => {
        $( $out )*
    };
    (   @ins( async $( $ins:tt )* )
        @out( $( $out:tt )* )
    ) => {
        $crate::strip_async!{
            @ins( $( $ins )* )
            @out( $( $out )* )
        }
    };
    (   @ins( .await $( $ins:tt )* )
        @out( $( $out:tt )* )
    ) => {
        $crate::strip_async!{
            @ins( $( $ins )* )
            @out( $( $out )* )
        }
    };
    (   @ins( $token:tt $( $ins:tt )* )
        @out( $( $out:tt )* )
    ) => {
        $crate::strip_async!{
            @ins( $( $ins )* )
            @out( $( $out )* $token )
        }
    };
    (
        $( ins:tt )*
    ) => {

        $crate::strip_async!{
            @ins( $( $ins )* )
            @out(  )
        }
    };
}
