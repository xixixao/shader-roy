use crate::prelude::*;

// pub fn pixel_color(coordinates: float2, size: float2) -> float4 {
//   let _x = float2(2.0, 2.0);
//   let _y = test();
//   let mut d: float = 1000.0;
//   d = d.min(100.0);

//   // d = min(d, sdCircle(p, vec2(-0.1, 0.4), 0.15));
//   // d = min(d, sdCircle(p, vec2( 0.5, 0.1), 0.35));

//   // return d;
//   float4(coordinates.x / size.x, coordinates.y / size.y, 0.0, 1.0)
// }

// fn test() -> float2 {
//   float2(1.0, 2.0)
// }

pub fn pixel_color(coordinates: Float2, size: Float2) -> Float4 {
  // project screen coordinate into world
  let p: Float2 = screen_to_world(coordinates, size);
  float4(0.0, 0.0, 0.0, 0.0)
}
//     // signed distance for scene
//     float sd = sdf(p);
//     // compute signed distance to a colour
//     vec3 col = shade(sd);
//     fragColor = vec4(col, 1.0);
// }

// // --- SDF utility library

// float sdCircle(in Float2 p, in Float2 pos, float radius)
// {
//     return length(p-pos)-radius;
// }

// float sdBox(in Float2 p, in Float2 pos, in Float2 size)
// {
//     Float2 d = abs(p-pos)-size;
//     return min(0.0, max(d.x, d.y))+length(max(d,0.0));
// }

// // polynomial smooth min (k = 0.1);
// float sminCubic(float a, float b, float k)
// {
//     float h = max(k-abs(a-b), 0.0);
//     return min(a, b) - h*h*h/(6.0*k*k);
// }

// float opU(float d1, float d2)
// {
//     return min(d1, d2);
// }

// float opBlend(float d1, float d2)
// {
//     float k = 0.2;
//     return sminCubic(d1, d2, k);
// }

// // --- Misc functions

// // https://www.shadertoy.com/view/ll2GD3
// vec3 palette(in float t, in vec3 a, in vec3 b, in vec3 c, in vec3 d)
// {
//     t = clamp(t, 0., 1.);
//     return a + b*cos(6.28318*(c*t+d));
// }

fn screen_to_world(screen: Float2, size: Float2) -> Float2 {
  let mut result: Float2 = 2.0 * (screen / size - 0.5);
  // result.x *= size.x/size.y;
  // result
  float2(0.0, 0.0)
}

// vec3 shade(float sd)
// {
//     float maxDist = 2.0;
//     vec3 palCol = palette(clamp(0.5-sd*0.4, -maxDist,maxDist),
//                       vec3(0.3,0.3,0.0),vec3(0.8,0.8,0.1),vec3(0.9,0.7,0.0),vec3(0.3,0.9,0.8));

//     vec3 col = palCol;
//     // Darken around surface
// 	col = mix(col, col*1.0-exp(-10.0*abs(sd)), 0.4);
// 	// repeating lines
//     col *= 0.8 + 0.2*cos(150.0*sd);
//     // White outline at surface
//     col = mix(col, vec3(1.0), 1.0-smoothstep(0.0,0.01,abs(sd)));
//     return col;
// }
