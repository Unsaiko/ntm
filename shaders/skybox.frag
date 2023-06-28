#version 400

in vec3 texCoords;

uniform samplerCube skybox;

out vec4 FragColor;

void main() 
{
    FragColor = texture(skybox, texCoords);
}