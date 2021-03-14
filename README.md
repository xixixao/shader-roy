# ShaderRoy

ShaderToy clone in Rust, currently supporting MacOS.

## Features

1. `cargo run <rust project dir>` displays a single macOS window filled with a [Metal](https://developer.apple.com/metal/) framework [fragment shader](https://developer.apple.com/documentation/metal/using_a_render_pipeline_to_render_primitives#3682806).
2. You can edit and save the Rust project source code (in VS code or any other editor) to change the fragment shader output and the window will update in real time.
3. You write the shader in Rust but it is compiled to [Metal Shading Language](https://developer.apple.com/metal/Metal-Shading-Language-Specification.pdf) (a variation of C++)
4. In the shader source you can reference the `const` `INPUT` struct which provides inputs for each frame, similarly to _Input Uniforms_ in ShaderToy. You don't need to thread these values through your functions as arguments, despite Metal having no concept of global uniforms like WebGL does.
5. You can split the shader across multiple files using `mod <name>` and `use <name>::*`.
6. You can pause, restart and even run another shader file from the command line while the window is open.

## Instructions

1. clone this repo
2. run `cargo run raymarching_eyes`
3. edit `examples/raymarching_eyes.rs`

Why Rust for the shader source? Better syntax, better editor integration and because it's a fun hack. It should feel exactly like writing Rust (which feels awesome!). Unlike in ShaderToy the Rust typechecker warns immediately about most errors one might make.

## Metal Shading Rust Language

In general you will write Rust that closely resembles the C++ Metal Shading Language API, except for a few differences.

The `INPUT` struct is documented in [shader_roy_metal_sl_interface](shader_roy_metal_sl_interface/src/shader_roy_metal_sl_interface.rs).

<table>
<tr>
<td> API </td> <td> MSL (C++) </td> <td> Rust </td>
</tr>

<tr>
<td> Scalar Types </td>
<td>

```cpp
float foo() {
  return 3.0;
}
```

</td>
<td>

Use standard Rust types that correspond to the Metal types.

```rust
fn foo() -> f32 {
  3.0
}
```

</td>
</tr>

<tr>
<td> Vector Types </td>
<td>

```cpp
void foo(float3 a, uint2 b) {}
```

</td>
<td>

Use the generic `Vec` type. Its default type argument is `f32`, for convenience.

```rust
fn foo(a: Vec3, b: Vec2<u32>) {}
```

</td>
</tr>

<tr>
<td> Constructors </td>
<td>

vector constructors (`float4` etc.) can take arbitrary number of vector/scalar arguments, as long as they combine to the right length vector

```cpp
auto x = float2(1.0);
float4(x, x);

uint2(0, 0);
```

</td>
<td>

In Rust, these follow the type names. You need to call these as methods.

```rust
let x = 1.0.vec2();
(x, x).vec4();

vec2u32(0, 0);
```

</td>
</tr>

<tr>
<td> Access Constructors </td>
<td>

vector component selection constructors (`xxy` etc.) allows permutation and/or repetition of components:

```cpp
auto pos = float2(1.0, 2.0);
auto foo = pos.yyxy;
// => float4(2.0, 2.0, 1.0, 2.0)
```

</td>
<td>

In Rust you need to call these as methods:

```rust
let pos = (1.0, 2.0).vec2();
let foo = pos.yyxy();
// => (2.0, 2.0, 1.0, 2.0).vec4()
```

</td>
</tr>

<tr>
<td> Functions </td>
<td>

```cpp
min(x, y);
```

</td>
<td>

Math, geometric and common functions need to be called as methods

```rust
x.min(y)
```

</td>
</tr>

<tr>
<td> Renamed functions </td>
<td>

```cpp
clamp(x, 0.3, 0.4);
length(x);
length_squared(x);
normalize(x);
faceforward(x, incident, ref);
reflect(x, normal);
refract(x, normal, eta);
fmin(x, y);
fmax(x, y);
```

</td>
<td>

Names follow [vek](https://docs.rs/vek/0.13.1/vek/vec/repr_c/vec3/struct.Vec3.html)

```rust
x.clamped(0.3, 0.4);
x.magnitude();
x.magnitude_squared();
x.normalized();
x.face_forward(incident, ref);
x.reflected(normal);
x.refracted(normal, eta);
x.min(y);
x.max(y);
```

</td>
</tr>

<tr>
<td> Methods with different argument order </td>
<td>

```cpp
mix(0.3, 0.4, a);
smoothstep(0.3, 0.4, x);
step(0.3, x);
```

</td>
<td>

When one argument is special from the others it is used as the receiver of the method call.

```rust
a.mix(0.3, 0.4);
x.smoothstep(0.3, 0.4);
x.step(0.3);
```

</td>
</tr>

</table>

## Limitations

### Modules

Right now only modules in the same directory as the main file will be watched.
Only files in the same directory are supported for `mod <name>`s, `<name>/mod.rs` is not supported.
There is no support for `path` attribute on `mod`s.

### Let bindings

Variables cannot be redeclared. (_for now_)

### Constructors

In Rust we could use a single generic `vec2` constructor for all `Vec2`s. But this would require actually using `rustc` to compile the constructors to the concrete Metal C++ constructors. To keep things simpler, the Rust bindings here require specifying the constructors directly (`vec2bool` -> `bool2`).

## Development

Print the compiled shader without opening the window:

```
cargo test -- --nocapture
```
