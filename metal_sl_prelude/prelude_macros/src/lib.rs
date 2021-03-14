pub extern crate prelude_proc_macros;

#[macro_export]
macro_rules! implement_constructors {
  ([$($type_name:ident),*], $template:tt) => {
    $(
      $crate::implement_constructors_for_type! {
        $type_name, $template
      }
    )*
  };
}

#[macro_export]
macro_rules! implement_constructors_for_type {
  ($type_name:ident, {$($type:ident => $set:tt),*$(,)?}) => {
    $(
      $crate::prelude_proc_macros::define_trait! { $type_name $type }
      $crate::implement_constructor_trait! {
        $type_name: $type => $set
      }
    )*
  };
}

#[macro_export]
macro_rules! implement_constructor_trait {
  ($type_name:ident: $type:ident => {$($arg_list:tt),*$(,)?}) => {
    $(
      $crate::implement_constructor_trait! {
        $type_name: $type => $arg_list
      }
    )*
  };
  ($type_name:ident: $type:ident => ($($arg_name:ident: $arg_type:ident),*$(,)?)) => {
    $crate::prelude_proc_macros::implement_trait! {
      $type_name $type ($($arg_name: $arg_type),*)
    }
  }
}

#[macro_export]
macro_rules! implement_accessors {
  ($($type:ident),*$(,)?) => {
    $(
      $crate::prelude_proc_macros::implement_accessors! { $type }
    )*
  };
}
