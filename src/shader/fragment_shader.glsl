#version 450 core

out vec4 Color;

in vec3 vertex_uv;

uniform float u_aspect_ratio;


float circle(vec2 loc, vec2 center, float radius);

void main()
{   
    vec2 uv = vertex_uv.xy / 2. + 0.5;

    vec3 color0 = vec3(0.1, 0.8, 1.0);

    float circle0 = circle(
        uv,
        vec2(0.5, 0.5), 
        0.1
    );

    Color = vec4(color0 * circle0, 1.0);
}

float circle(vec2 loc, vec2 center, float radius) {
    loc -= center;

    loc.x *= u_aspect_ratio;

    float delta = 0.005;

    return 1.0 - smoothstep(
        radius * (radius - delta),
        radius * (radius  + delta),
        dot(loc, loc)
    );
}