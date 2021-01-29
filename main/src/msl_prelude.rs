#![allow(dead_code)]

mod access;
mod common;
mod construct;
mod generic;
mod geometric;
mod math;
mod types;

pub use access::*;
pub use common::*;
pub use construct::*;
pub use generic::*;
pub use geometric::*;
pub use math::*;
pub use types::*;

pub struct PixelInput {
  /// Window size in physical units (at the native resolution of the display device)
  pub window_size: Float2,
  /// Time since starting the program, in fractions of seconds.
  pub elapsed_time_secs: Float,
}
