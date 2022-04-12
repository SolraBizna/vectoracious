/* -*- glsl -*- */

in mediump vec2 uv;

uniform sampler2D src1;
uniform sampler2D src2;

out mediump vec4 result;

void main() {
  vec3 sample = max(texture(src1, uv).rgb, vec3(0.0, 0.0, 0.0))
    + max(texture(src2, uv).rgb, vec3(0.0, 0.0, 0.0));
  result = vec4(sample, 1.0);
}
