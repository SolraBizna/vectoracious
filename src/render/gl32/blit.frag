/* -*- glsl -*- */

in mediump vec2 uv;

uniform sampler2D src;

out mediump vec4 result;

void main() {
  vec3 sample = texelFetch(src, ivec2(uv), 0).rgb;
  result = vec4(sample, 1.0);
}
