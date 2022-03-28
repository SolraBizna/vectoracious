/* -*- c -*- */

out lowp vec4 result;

flat in lowp vec4 frag_fillcolor;
flat in lowp vec4 frag_strokecolor;
in mediump vec2 uv;
uniform sampler2D atlas;

// yeah, I stole it from Mr. Chlumský's thesis, page 46
float median(float a, float b, float c) {
  return max(min(a,b),min(max(a,b),c));
}

float sample_d(vec2 uv) {
  vec3 sample = texture(atlas, uv).rgb;
  return median(sample.r, sample.g, sample.b);
}

float take_sample(vec2 uv) {
  float d = sample_d(uv);
  return clamp((d-0.5)/fwidth(d) + 0.5, 0.0, 1.0);
}

const float BOX_SCALE = 0.25;

void main() {
  vec2 delta_u = BOX_SCALE * dFdx(uv.xy);
  vec2 delta_v = BOX_SCALE * dFdy(uv.xy);
  float alpha
    = (take_sample(uv.xy - delta_u - delta_v)
       + take_sample(uv.xy + delta_u - delta_v)
       + take_sample(uv.xy - delta_u + delta_v)
       + take_sample(uv.xy + delta_u + delta_v)) * 0.25;
  float buzz = sample_d(uv) * 2.0;
  result = mix(frag_strokecolor * buzz, frag_fillcolor, alpha);
}
