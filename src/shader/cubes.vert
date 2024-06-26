#version 450 core

layout (location = 0) in vec2 position;
// layout (location = 1) in vec3 color;
// layout (location = 1) in vec2 tex_coord;

out vec4 vertex_uv;
// out vec2 vertex_texture_coord;

uniform mat4 u_model; 
uniform mat4 u_view; 
uniform mat4 u_projection; 
    
void main()
{
    // vertex_texture_coord = tex_coord;
    vertex_uv = vec4(position, 0.0, 1.0);
    gl_Position = vec4(position, 0.0, 1.0);;
}
