/* -*- glsl -*- */

#define MAX_COLORS 16 // sync this with the Rust side!

in mediump vec2 pos;
in uint color_index;

uniform mat3 transform;
uniform float opacity;
uniform vec4 colors[MAX_COLORS];

flat out lowp vec4 frag_color;

void main() {
  vec3 transformed = transform * vec3(pos, 1.0);
  gl_Position = vec4(transformed.xy, 0.0, 1.0);
  frag_color = colors[color_index] * opacity;
}
