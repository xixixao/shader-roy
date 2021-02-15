#include <metal_stdlib>

using namespace metal;

// Input passed from CPU to GPU on each frame, defined for Rust
// version of the shader code in shader_roy_metal_sl_interface.
typedef struct {
    float2 window_size;
    float elapsed_time_secs;
} Input;

// The function that has to be implemented by each ShaderRoy shader.
float4 pixel_color(float2 coordinates, Input input);

// The vertex shader
float2 rect_vert(float4 rect, uint vid);
vertex float4 vertex_shader(
    const device float4 *clear_rect [[ buffer(0) ]],
    unsigned int vertex_index [[ vertex_id ]]
) {
    return float4(rect_vert(*clear_rect, vertex_index), 0, 1);
}

// The fragment shader
fragment float4 fragment_shader(
    float4 in [[position]],
    constant Input &input [[ buffer(0) ]]
) {
    return pixel_color(float2(in.x, in.y), input);
}

// Helper for computing pixel position in vertext shader
float2 rect_vert(float4 rect, uint vertex_index) {
    float left = rect.x;
    float right = rect.x + rect.z;
    float bottom = rect.y;
    float top = rect.y + rect.w;

    switch (vertex_index) {
        case 0:
            return float2(right, top);
        case 1:
            return float2(left, top);
        case 2:
            return float2(right, bottom);
        case 3:
            return float2(left, bottom);
    }
}