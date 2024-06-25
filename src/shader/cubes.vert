#version 450 core

layout (location = 0) in vec3 position;
// layout (location = 1) in vec3 color;
layout (location = 1) in vec2 tex_coord;

// out vec2 vertex_uv;
out vec2 vertex_texture_coord;

uniform mat4 u_model; 
uniform mat4 u_view; 
uniform mat4 u_projection; 
    
void main()
{
    vertex_texture_coord = tex_coord;
    gl_Position = u_projection * u_view * u_model * vec4(position, 1.0);
}