/* -*- glsl -*- */

in mediump vec2 uv;

uniform sampler2D src;
uniform sampler1D gauss;
uniform int num_samples;
uniform ivec2 max_uv;

out mediump vec4 result;

void main() {
  ivec2 center = ivec2(uv);
  highp vec3 accum = texelFetch(src, center, 0).rgb * texelFetch(gauss, 0, 0).r;
  for(int i = 1; i < num_samples; i += 1) {
#if defined(BLOOM_HORIZ)
    ivec2 off = ivec2(i, 0);
#elif defined(BLOOM_VERT)
    ivec2 off = ivec2(0, i);
#else
#error one of BLOOM_HORIZ or BLOOM_VERT must be defined
#endif
    accum += (texelFetch(src, clamp(center + off, ivec2(0.0), max_uv), 0).rgb
      + texelFetch(src, clamp(center - off, ivec2(0.0), max_uv), 0).rgb)
      * texelFetch(gauss, i, 0).r;
  }
  result = vec4(accum, 1.0);
}
