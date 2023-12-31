/* -*- glsl -*- */

in mediump vec2 uv;

uniform sampler2D src;
#ifdef WITH_MATRIX
uniform mat4x3 mat;
#endif
uniform ivec3 dim;

out mediump vec4 result;

void main() {
  ivec2 base = ivec2(uv) * dim.xy;
  vec3 sample = vec3(0.0, 0.0, 0.0);
  for(int y = 0; y < dim.y; y += 1) {
    for(int x = 0; x < dim.x; x += 1) {
      sample += texelFetch(src, base + ivec2(x, y), 0).rgb;
    }
  }
#ifdef WITH_MATRIX
  result = vec4(max(mat * vec4(sample / float(dim.z), 1.0),
                    vec3(0.0, 0.0, 0.0)), 1.0);
#else
  result = vec4(max(sample / float(dim.z), vec3(0.0, 0.0, 0.0)), 1.0);
#endif
}
