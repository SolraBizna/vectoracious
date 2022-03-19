#version 140 /* -*- c -*- */

in mediump vec2 pos;
in mediump vec2 uv_in;
in lowp vec4 vert_fillcolor;
in lowp vec4 vert_strokecolor;

flat out lowp vec4 frag_fillcolor;
flat out lowp vec4 frag_strokecolor;
out mediump vec2 uv;

void main() {
  gl_Position = vec4(pos,0.0,1.0);
  frag_fillcolor = vert_fillcolor;
  frag_strokecolor = vert_strokecolor;
  uv = uv_in;
}
