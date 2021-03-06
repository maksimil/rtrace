#version 330

layout (lines) in;
layout (triangle_strip, max_vertices = 3) out;

void main() {
    gl_Position = vec4(0.0, 0.0, 0.0, 1.0);
    EmitVertex();

    gl_Position = gl_in[0].gl_Position;
    EmitVertex();

    gl_Position = gl_in[1].gl_Position;
    EmitVertex();

    EndPrimitive();
}