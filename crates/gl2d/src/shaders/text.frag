precision mediump float;

uniform sampler2D texture;
varying vec4 frag_color;
varying vec2 coord2;

void main() {
    gl_FragColor = frag_color * mod(texture2D(texture, coord2).z * 255.0, 2.0);
}
