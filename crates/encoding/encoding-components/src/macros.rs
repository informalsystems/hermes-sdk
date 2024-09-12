#[macro_export]
macro_rules! HList {
  ( $(,)? ) => {
    ()
  };
  ( $e:ty ) => {
    ( $e, () )
  };
  ( $e:ty, $($tail:tt)* ) => {
    ( $e, $crate::HList!( $($tail)* ) )
  };
}

#[macro_export]
macro_rules! Sum {
  ( $(,)? ) => {
    $crate::types::either::Void
  };
  ( $e:ty ) => {
    $crate::types::either::Either<
      $e,
      $crate::types::either::Void
    >
  };
  ( $e:ty, $($tail:tt)* ) => {
    $crate::types::either::Either<
      $e,
      $crate::Sum!( $($tail)* )
    >
  };
}
