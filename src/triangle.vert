#version 330 core
layout (location = 0) in vec3 Position;
layout (location = 2) in vec2 aCoords;

uniform vec4 c;
uniform mat4 model;
uniform mat4 view;
uniform mat4 perspective;

out vec2 texCoords;
out vec4 ourColor;

void main()
{
    gl_Position = perspective * view * model * vec4(Position, 1.0);
    texCoords = aCoords;
    ourColor = c;
}
