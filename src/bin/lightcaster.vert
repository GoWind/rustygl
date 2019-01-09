#version 330 core

layout (location=0) in vec3 aPosition;
layout (location=1) in vec3 normal;
layout (location=2) in vec2 texCoords;

out vec2 TexCoords;
out vec3 Normal;
out vec3 FragPos;

uniform mat4 model;
uniform mat4 view;
uniform mat4 perspective;

void main()
{
    FragPos = vec3(model * vec4(aPosition, 1.0));
    Normal = mat3(transpose(inverse(model))) * normal;
    TexCoords = texCoords;

    gl_Position = perspective * view * vec4(FragPos, 1.0);
}