#version 400
layout (location = 0) in vec3 pos;

uniform mat4 projection;
uniform mat4 view;

out vec3 texCoords;

void main() 
{
    texCoords = vec3(pos.x, pos.y, -pos.z);
    vec4 tpos = projection * view * vec4(pos, 1.0);
    gl_Position = vec4(tpos.x, tpos.y, tpos.w, tpos.w);
}