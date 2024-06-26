#version 450 core

in vec3 v_normal;
in vec3 v_fragment_position;

layout(location=0) out vec4 color;

uniform vec3 u_light_color;
uniform vec3 u_object_color;

uniform vec3 u_light_position;
uniform vec3 u_view_position;


void main()
{ 
    float ambient_strength = 0.1;
    vec3 ambient = ambient_strength * u_light_color;


    vec3 norm = normalize(v_normal);
    vec3 light_direction = normalize(u_light_position - v_fragment_position);

    float diff = max(dot(norm, light_direction), 0.0);
    vec3 diffuse = diff * u_light_color;

    float specular_strength = 0.5;
    vec3 view_direction = normalize(u_view_position - v_fragment_position);
    vec3 reflect_direction = reflect(-light_direction, norm);
    float spec = pow(max(dot(view_direction, reflect_direction), 0.0), 32);
    vec3 specular = specular_strength * spec * u_light_color;

    vec3 result = (ambient + diffuse + specular) * u_object_color;

    color = vec4(result, 1.0);
}