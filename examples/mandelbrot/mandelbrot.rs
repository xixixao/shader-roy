use shader_roy_metal_sl_interface::*;

pub fn pixel_color(coordinates: Float2) -> Float4 {
  let Float2 { x: cx, y: cy } = screen_to_world(coordinates);
  let mut x: Float = 0.0;
  let mut y = 0.0;
  let mut iteration = 0;
  let max_iteration = 1000;
  while (x * x + y * y) <= 4.0 && iteration < max_iteration {
    let xtemp = x * x - y * y + cx;
    y = 2.0 * x * y + cy;
    x = xtemp;
    iteration += 1;
  }

  (iteration as Float / 80.0).float4()
}

fn screen_to_world(screen: Float2) -> Float2 {
  let size = INPUT.window_size;
  let mut center = 2.0 * (INPUT.cursor_position / size - 0.5);
  center.x *= size.x / size.y;
  center.y *= -1.0;

  // let center = float2(0.0, 1.0);
  // let time = 1.0;
  let time = INPUT.elapsed_time_secs.fmod(10.0) + 1.0;
  let zoom = 1.0 / time.pow(time / 2.0);
  let mut result = 2.0 * (screen / size - 0.5);
  result.x *= size.x / size.y;
  result.y *= -1.0;
  result -= center;
  result *= zoom;
  result += center;
  result
}
