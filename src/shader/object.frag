#version 450 core

layout(location=0) out vec4 color;

uniform vec3 u_light_color;
uniform vec3 u_object_color;


void main()
{ 
    float ambient_strength = 0.1;
    vec3 ambient = ambient_strength * u_light_color;

    vec3 result = ambient * u_object_color;
    color = vec4(result, 1.0);
}