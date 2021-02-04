#version 140
in vec2 ray;
in float rad;

void main() { gl_Position = vec4(ray * rad, 0.0, 1.0); }