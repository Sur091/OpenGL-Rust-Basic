#version 450 core

in vec3 v_normal;
in vec3 v_fragment_position;

layout(location=0) out vec4 color;

uniform vec3 u_light_color;
uniform vec3 u_object_color;

uniform vec3 u_light_position;


void main()
{ 
    float ambient_strength = 0.1;
    vec3 ambient = ambient_strength * u_light_color;


    vec3 norm = normalize(v_normal);
    vec3 light_direction = normalize(u_light_position - v_fragment_position);

    float diff = max(dot(norm, light_direction), 0.0);
    vec3 diffuse = diff * u_light_color;

    vec3 result = (ambient + diffuse) * u_object_color;

    color = vec4(result, 1.0);
}