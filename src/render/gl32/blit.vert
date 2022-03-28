/* -*- c -*- */

in mediump vec2 pos;
in mediump vec2 uv_in;
out vec2 uv;

void main() {
  gl_Position = vec4(pos, 0.0, 1.0);
  uv = uv_in;
}
