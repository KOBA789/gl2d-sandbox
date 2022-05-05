attribute vec4 vert_position;
attribute vec4 vert_color;
uniform   mat4 projection;
varying   vec4 frag_color;
varying   vec2 coord2;

void main() {
    gl_Position = projection * vec4(vert_position.xy, 0, 1);
    frag_color = vert_color;
    coord2 = vert_position.zw;
}
