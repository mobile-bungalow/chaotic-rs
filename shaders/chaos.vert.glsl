#version 400

in vec3 position;
in vec3 color;
out vec4 color_in;

void main() {
    color_in = vec4(color, 1.0);
    gl_Position = vec4(position, 1.0);
}