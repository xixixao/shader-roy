pub use metal_sl_prelude::*;

/// This is the `constant` information passed to the pixel shader from ShaderRoy on each frame.
#[derive(Copy, Clone)]
pub struct Input {
  /// Window size in physical units (at the native resolution of the display device).
  pub window_size: Vec2,
  /// Window's top left corner position in physical units.
  pub window_position: Vec2,
  /// The cursor left (x) and top (y) position in physical units.
  pub cursor_position: Vec2,
  /// Whether the cursor is inside the window and the window is focused.
  ///
  /// Defaults to false until the cursor moves inside or enters the window.
  pub is_cursor_inside_window: bool,
  /// Time since starting the program, in fractions of seconds.
  pub elapsed_time_secs: f32,
  /// Time since the rendering of the previous frame.
  pub elapsed_time_since_last_frame_secs: f32,
  /// Number of frames rendered so far, starting with 1.
  pub frame_count: f32,
  /// Local calendar year, month, day. tz is UTC timezone offset in secs.
  pub year_month_day_tz: Vec4,
}

// The values here are dummy values for type checking, real values are passed at runtime.
pub const INPUT: Input = Input {
  window_size: Vec2 { x: 0.0, y: 0.0 },
  window_position: Vec2 { x: 0.0, y: 0.0 },
  cursor_position: Vec2 { x: 0.0, y: 0.0 },
  is_cursor_inside_window: false,
  elapsed_time_secs: 0.0,
  elapsed_time_since_last_frame_secs: 0.0,
  frame_count: 0.0,
  year_month_day_tz: Vec4 {
    x: 1970.0,
    y: 12.0,
    z: 24.0,
    w: 1.0,
  },
};
