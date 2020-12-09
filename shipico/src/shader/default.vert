#version 100
attribute vec2 pos;
attribute vec4 color0;

varying lowp vec4 color;

uniform mat4 mvp;

void main() {
    gl_Position = mvp * vec4(pos, 0.0, 1.0);
    color = color0;
}
