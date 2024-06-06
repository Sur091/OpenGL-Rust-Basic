#version 450 core

out vec4 Color;

in vec3 vertex_uv;

uniform float u_aspect_ratio;
uniform float u_time;

float polygon(vec2 loc, vec2 center, int sides, float raidus, float theta);
float circle(vec2 loc, vec2 center, float radius);
mat2 rotate2D(float theta);
float random (in vec2 _st);

void main()
{   
    vec2 uv = vertex_uv.xy + 0.5;
    uv.x *= u_aspect_ratio;

    uv = fract(uv * 10.);

    uv -= 0.5;
    uv = rotate2D(sin(u_time)) * uv;
    uv += 0.5;

    vec3 color0 = vec3(0.1, 0.8, 1.0);
    // vec3 color0 = vec3(0.);
    float circle0 = circle(
        uv,
        rotate2D(sin(u_time)) * vec2(0.8, 0.2), 
        0.1
    );

    vec3 color1 = vec3(0.98, 0.36, 0.0);
    float pentagon1 = polygon(
        uv,
        vec2(0.5, 0.8),
        5,
        0.2,
        3.14/4.
    );

    vec3 color2 = vec3(0.1, 0.92, 0.07);
    float triangle2 = polygon(
        uv, 
        vec2(0.3, 0.5),
        3,
        0.3,
        0.
    );

    Color = vec4(color0 * circle0 + color1 * pentagon1 + color2 * triangle2, 1.0);
}

float polygon(vec2 loc, vec2 center, int sides, float raidus, float theta) {
    loc -= center;

    // Angle and radius from the current pixel
    float a = atan(loc.x,loc.y)+3.14159265358979 + theta;
    float r = 6.28318530718/float(sides);

    // Shaping function that modulate the distance
    float pct = cos(floor(.5+a/r)*r-a)*length(loc);
    
    float delta = 0.01;
    return 1.-smoothstep(raidus-delta, raidus, pct);
}

float circle(vec2 loc, vec2 center, float radius) {
    loc -= center;

    float delta = 0.005;

    return 1.0 - smoothstep(
        radius * (radius - delta),
        radius * (radius  + delta),
        dot(loc, loc)
    );
}

mat2 rotate2D(float theta) {
    return mat2 (
        cos(theta), sin(theta),
        -sin(theta), cos(theta)
    );
}

float random (in vec2 _st) {
    return fract(sin(dot(_st.xy,
                         vec2(12.9898,78.233)))*
        43758.5453123);
}