/* -*- glsl -*- */

flat in lowp vec4 frag_color;

out lowp vec4 result;

void main() {
  result = frag_color;
}
