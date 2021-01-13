#include <metal_stdlib>
// #include <metal_stdlib>

using namespace metal;

// typedef struct {
// 	packed_float2 position;
// 	packed_float3 color;
// } vertex_t;

struct ColorInOut {
    float4 position [[position]];
    float4 color;
};

struct Rect {
    float x;
    float y;
    float w;
    float h;
};

struct Color {
    float r;
    float g;
    float b;
    float a;
};

typedef struct {
    uint width;
    uint height;
} Size;

struct ClearRect {
    Rect rect;
    Color color;
};

float2 rect_vert(
    Rect rect,
    uint vid
) {
    float2 pos;

    float left = rect.x;
    float right = rect.x + rect.w;
    float bottom = rect.y;
    float top = rect.y + rect.h;

    switch (vid) {
    case 0:
        pos = float2(right, top);
        break;
    case 1:
        pos = float2(left, top);
        break;
    case 2:
        pos = float2(right, bottom);
        break;
    case 3:
        pos = float2(left, bottom);
        break;
    }
    return pos;
}

vertex ColorInOut clear_rect_vertex(
    const device ClearRect *clear_rect [[ buffer(0) ]],
    unsigned int vid [[ vertex_id ]]
) {
    ColorInOut out;
    float4 pos = float4(rect_vert(clear_rect->rect, vid), 0, 1);
    auto col = clear_rect->color;

    out.position = pos;
    out.color = float4(col.r, col.g, col.b, col.a);
    return out;
}


float2 len(float2 p) {
    return sqrt(p.x*p.x + p.y*p.y);
}

fragment float4 clear_rect_fragment(
    ColorInOut in [[stage_in]],
    constant Size &size [[ buffer(0) ]]
)
{
    float4 p_abs_4 = in.position;
    float2 p_abs = float2(p_abs_4.x, p_abs_4.y);
    float2 s = float2(size.width, size.height);
    float2 p = (p_abs - s / 2.0) / s;
    return float4(smoothstep(0.1, 0.12, len(p)), 0, 1.0);
};
