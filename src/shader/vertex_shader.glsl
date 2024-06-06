#version 450 core

layout (location = 0) in vec3 position;

out vec3 vertex_uv;
    
void main()
{
    vertex_uv = position;
    gl_Position = vec4(position, 1.0);
    // gl_Position = vec4(position.xyz, 1.0);
    // gl_Position = vec4(position.x, position.y, position.z, 1.0);
}