#version 400

// layout(location = 2) uniform matrix {
//     mat4 matrix;
// } mat;



layout(location = 0) in vec3 position;
layout(location = 1) in vec3 color;
out vec4 color_in;



void main() {
    color_in = vec4(color, 1.0);
    gl_Position = vec4(position.xy * 0.001, 1.0, 1.0);
}