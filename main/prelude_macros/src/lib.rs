pub extern crate prelude_proc_macros;

#[macro_export]
macro_rules! implement_constructors {
  ($($type:ident => $nums:tt => $set:tt),*$(,)?) => {
    $(
      $crate::define_trait! {
        $type => $nums
      }
      $crate::implement_trait! {
        $type => $set
      }
    )*
  };
}

#[macro_export]
macro_rules! define_trait {
  ($type:ident => [$($num:literal),*]) => {
    $($crate::prelude_proc_macros::define_trait! {
      $type $num
    })*
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
  ($type:ident => ($first_arg:ident $(, $arg_name:ident: $arg_type:ident)*)) => {
    $crate::prelude_proc_macros::implement_trait! {
      $type $first_arg ($($arg_name: $arg_type),*)
    }
  }
}
