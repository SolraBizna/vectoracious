/* -*- glsl -*- */

in mediump vec2 uv;

uniform sampler2D src;

out mediump vec4 result;

void main() {
  vec3 sample = texture(src, uv).rgb;
  result = vec4(sample, 1.0);
}
