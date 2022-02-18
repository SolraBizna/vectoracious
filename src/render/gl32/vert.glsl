#version 130 /* -*- c -*- */

in mediump vec2 pos;
in lowp vec4 vert_color;

flat out lowp vec4 frag_color;

void main() {
  gl_Position = vec4(pos,0.0,1.0);
  frag_color = vert_color;
}
