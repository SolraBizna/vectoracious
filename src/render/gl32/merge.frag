/* -*- glsl -*- */

in mediump vec2 uv;

uniform sampler2D src1;
uniform sampler2D src2;

out mediump vec4 result;

void main() {
  vec3 sample = max(texelFetch(src1, ivec2(uv), 0).rgb, vec3(0.0, 0.0, 0.0))
    + max(texelFetch(src2, ivec2(uv), 0).rgb, vec3(0.0, 0.0, 0.0));
  result = vec4(sample, 1.0);
}
