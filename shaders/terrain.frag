#version 400

in vec2 texCoord;
in float height;

out vec4 FragColor;

void main()
{
   FragColor = vec4(height, height, height, 1.0);
}