#version 100
precision mediump float;
varying lowp vec4 ppos;

void main() {
    // gl_FragColor = vec4(1.0, 1.0, 1.0, 1.0);
    gl_FragColor = abs(ppos);
}
