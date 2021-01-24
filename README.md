# ShaderRoy

ShaderToy clone in Rust, currently supporting MacOS.

## Contents

1. `cargo run` displays a single macOS window filled with a [Metal](https://developer.apple.com/metal/) framework [fragment shader](https://developer.apple.com/documentation/metal/using_a_render_pipeline_to_render_primitives#3682806).
2. You can edit and save `shader.rs` (in VS code or any other editor) to change the fragment shader output and the window will update in real time.
3. `shader.rs` is written in Rust but complies to [Metal Shading Language](https://developer.apple.com/metal/Metal-Shading-Language-Specification.pdf) (a variation of C++)

## Instructions

1. clone this repo
2. run `cargo run`
3. edit `shader.rs`

`/examples` directory includes valid shaders you can copy over into `shader.rs`.

Why are we using Rust for the shader? Better syntax, better editor integration and because it's a fun hack. It should feel exactly like writing Rust (which feels awesome!). Unlike in ShaderToy the Rust typechecker warns you immediately about most errors you'd might make.

## Metal Shading Rust Language

In general you will write Rust that closely resembles the C++ Metal Shading Language API, except for a few differences:

<table>
<tr>
<td> API </td> <td> C++ </td> <td> Rust </td>
</tr>
<tr>
<td> Constructors </td>
<td>

vector constructors (`float4` etc.) can take arbitrary number of vector/scalar arguments, as long as they combine to the right length vector

```cpp
auto x = float2(1.0);
float4(x, x)
```

</td>
<td>

In Rust you need to call these as methods:

```rust
let x = 1.0.float2();
(x, x).float4()
```

</td>
</tr>

<tr>
<td> Clamp </td>
<td>

```cpp
clamp(x, 0.3, 0.4);
```

</td>
<td>

```rust
clamp(x, 0.3, 0.4);
// Notice "clamped", not "clamp"
x.clamped(0.3, 0.4);
```

</td>
</tr>
</table>

## Development

Print the compiled shader without opening the window:

```
cargo test -- --nocapture
```
