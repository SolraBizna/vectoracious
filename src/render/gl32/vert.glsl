#version 140 /* -*- c -*- */

in mediump vec2 pos;
in mediump vec2 uv_in;
in lowp vec4 vert_color;
in lowp vec4 vert_color2;

flat out lowp vec4 frag_color;
flat out lowp vec4 frag_color2;
out mediump vec2 uv;

void main() {
  gl_Position = vec4(pos,0.0,1.0);
  frag_color = vert_color;
  frag_color2 = vert_color2;
  uv = uv_in;
}
