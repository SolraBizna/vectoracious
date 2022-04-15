/* -*- glsl -*- */

in mediump vec2 uv;

uniform sampler2D src1;
uniform sampler2D src2;
#ifdef WITH_MATRIX
uniform mat4x3 mat1;
#endif

out mediump vec4 result;

void main() {
  vec3 sample = 
#ifdef WITH_MATRIX
    (mat1 *
     vec4(max(texture(src1, uv).rgb, vec3(0.0, 0.0, 0.0)), 1.0)).rgb
#else
    max(texture(src1, uv).rgb, vec3(0.0, 0.0, 0.0))
#endif
    + max(texture(src2, uv).rgb, vec3(0.0, 0.0, 0.0));
  result = vec4(sample, 1.0);
}
