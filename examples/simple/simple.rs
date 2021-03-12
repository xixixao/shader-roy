use shader_roy_metal_sl_interface::*;

pub fn pixel_color(coordinates: Float2) -> Float4 {
  let uv = coordinates / INPUT.window_size; // 0 <-> 1
  uv -= 0.5; // -0.5 <-> 0.5
  uv.x *= INPUT.window_size.x / INPUT.window_size.y; // make uv uniform

  let d = uv.magnitude(); // distance to center
  let radius = 0.3;
  let c = d.smoothstep(0.3, 0.29); // inverted to start in white
  (c.float3(), 1.0).float4()
}
