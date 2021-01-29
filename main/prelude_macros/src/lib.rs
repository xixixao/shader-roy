pub extern crate prelude_proc_macros;

#[macro_export]
macro_rules! implement_constructors {
  ($($type:ident => $set:tt),*$(,)?) => {
    $(
      $crate::prelude_proc_macros::define_trait! { $type }
      $crate::implement_trait! {
        $type => $set
      }
    )*
  };
}

#[macro_export]
macro_rules! implement_trait {
  ($type:ident => {$($arg_list:tt),*$(,)?}) => {
    $(
      $crate::implement_trait! {
        $type => $arg_list
      }
    )*
  };
  ($type:ident => ($($arg_name:ident: $arg_type:ident),*$(,)?)) => {
    $crate::prelude_proc_macros::implement_trait! {
      $type ($($arg_name: $arg_type),*)
    }
  }
}

#[macro_export]
macro_rules! implement {
  ($trait:path > $first_type:ty $(, $type:tt)* { $($implementation:item)* }) => {
    impl $trait for $first_type {
      $($implementation)*
    }
    $crate::implement! {
      $trait > $($type),* {
        $($implementation)*
      }
    }
  };
  ($trait:path > $_:tt) => {};
}

#[macro_export]
macro_rules! implement_accessors {
  ($($type:ident),*$(,)?) => {
    $(
      $crate::prelude_proc_macros::implement_accessors! { $type }
    )*
  };
}
