#version 450 core


// in vec2 vertex_uv;
in vec2 vertex_texture_coord;

layout(location=0) out vec4 color;

// uniform float u_aspect_ratio;
// uniform float u_time;
uniform sampler2D u_texture;

// float polygon(vec2 loc, vec2 center, int sides, float raidus, float theta);
// float circle(vec2 loc, vec2 center, float radius);
// mat2 rotate2D(float theta);
// float random (in vec2 _st);

// struct Ray {
//     vec3 origin;
//     vec3 direction;
// };

void main()
{   
    // vec2 uv = vertex_uv.xy + 0.5;
    // uv.x *= u_aspect_ratio;

    // Ray ray = Ray(vec3(0.0), vec3(1.0, 0.0, 0.0));

    // float r = ray.origin.x;
    // float g = uv.y;
    // float b = 0.0;


    vec4 tex_color = texture(u_texture, vertex_texture_coord);

    if (tex_color.z < 0.001) {
        tex_color = vec4(vertex_texture_coord.x, vertex_texture_coord.y, 0.2, 1.0);
    }

    color = tex_color;
}

// float polygon(vec2 loc, vec2 center, int sides, float raidus, float theta) {
//     loc -= center;

//     // Angle and radius from the current pixel
//     float a = atan(loc.x,loc.y)+3.14159265358979 + theta;
//     float r = 6.28318530718/float(sides);

//     // Shaping function that modulate the distance
//     float pct = cos(floor(.5+a/r)*r-a)*length(loc);
    
//     float delta = 0.01;
//     return 1.-smoothstep(raidus-delta, raidus, pct);
// }

// float circle(vec2 loc, vec2 center, float radius) {
//     loc -= center;

//     float delta = 0.005;

//     return 1.0 - smoothstep(
//         radius * (radius - delta),
//         radius * (radius  + delta),
//         dot(loc, loc)
//     );
// }

// mat2 rotate2D(float theta) {
//     return mat2 (
//         cos(theta), sin(theta),
//         -sin(theta), cos(theta)
//     );
// }

// float random (in vec2 _st) {
//     return fract(sin(dot(_st.xy,
//                          vec2(12.9898,78.233)))*
//         43758.5453123);
// }