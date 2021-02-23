pub use metal_sl_prelude::*;

/// This is the `constant` information passed to the pixel shader from ShaderRoy on each frame.
#[derive(Copy, Clone)]
pub struct Input {
  /// Window size in physical units (at the native resolution of the display device).
  pub window_size: Float2,
  /// Window's top left corner position in physical units.
  pub window_position: Float2,
  /// The cursor left (x) and top (y) position in physical units.
  pub cursor_position: Float2,
  /// Whether the cursor is inside the window and the window is focused.
  ///
  /// 1.0 for true, 0.0 for false.
  ///
  /// Defaults to false until the cursor moves inside or enters the window.
  pub is_cursor_inside_window: Float,
  /// Time since starting the program, in fractions of seconds.
  pub elapsed_time_secs: Float,
  /// Time since the rendering of the previous frame.
  pub elapsed_time_since_last_frame_secs: Float,
  /// Number of frames rendered so far, starting with 1.
  pub frame_count: Float,
  /// Local calendar year, month, day. tz is UTC timezone offset in secs.
  pub year_month_day_tz: Float4,
}

pub const INPUT: Input = Input {
  window_size: float2(0.0, 0.0),
  window_position: float2(0.0, 0.0),
  cursor_position: float2(0.0, 0.0),
  is_cursor_inside_window: 0.0,
  elapsed_time_secs: 0.0,
  elapsed_time_since_last_frame_secs: 0.0,
  frame_count: 0.0,
  year_month_day_tz: float4(0.0, 0.0, 0.0, 0.0),
};
