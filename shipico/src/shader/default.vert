#version 100
precision highp float;

attribute vec2 a_position;
// attribute vec3 a_norm;

uniform mat3 u_matrix;

varying lowp vec4 ppos;

void main() {
    // vec3 a = a_norm;
//   gl_Position = vec4((u_matrix * vec3(a_position, 1)).xy, 0, 1);
//   gl_Position = vec4(a_position, 0, 1);
    // vec2 pos = a_position.zy;
    gl_Position = vec4((u_matrix * vec3(a_position, 1)).xy, 0, 1);
    ppos = gl_Position;
}
