pub use metal_sl_prelude::*;

/// This is the `constant` information passed to the pixel shader from ShaderRoy on each frame.
#[derive(Copy, Clone)]
pub struct Input {
  /// Window size in physical units (at the native resolution of the display device)
  pub window_size: Float2,
  /// Time since starting the program, in fractions of seconds.
  pub elapsed_time_secs: Float,
}
