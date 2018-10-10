#version 330 core

in vec4 ourColor;
in vec2 texCoords;
out vec4 FragColor;
uniform vec4 c;
void main()
{
    FragColor = ourColor;

}
