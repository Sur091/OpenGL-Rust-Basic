#version 450 core


in vec4 vertex_uv;
// in vec2 vertex_texture_coord;

layout(location=0) out vec4 color;

uniform vec2 u_resolution;
uniform float u_aspect_ratio;


struct Ray {
    vec3 origin;
    vec3 direction;
};

vec3 ray_at(in Ray ray, float t) {
    return ray.origin + t * ray.direction;
}



float hit_sphere(in vec3 center, float radius, in Ray r) {
    vec3 oc = center - r.origin;
    float a = dot(r.direction, r.direction);
    float b = -2.0 * dot(r.direction, oc);
    float c = dot(oc, oc) - radius * radius;
    float discriminant = b*b - 4*a*c;
    
    if (discriminant < 0.0) {
        return -1.0;
    } else {
        return (-b - sqrt(discriminant)) / (2.0 * a);
    }
}

vec3 ray_color(in Ray r) {
    float t = hit_sphere(vec3(0.0, 0.0, -1.0), 0.5, r);
    if (t > 0.0) {
        vec3 N = normalize(ray_at(r, t) - vec3(0.0, 0.0, -1.0));
        return 0.5 * (N + 1.0);
    }
    vec3 unit_direction = normalize(r.direction);
    float a = 0.5 * (unit_direction.y + 1.0);
    return (1.0 - a) * vec3(1.0) + a * vec3(0.5, 0.7, 1.0);
}

void main()
{   
    vec2 uv = vec2(vertex_uv.x * u_aspect_ratio, vertex_uv.y);


    // Camera
    float focal_length = 1.0;
    float viewport_height = 2.0;
    float viewport_width = viewport_height * u_resolution.x / u_resolution.y;
    vec3 camera_center = vec3(0.0);

    // vectors across the horzontal and down the vertical viewport edges.
    vec3 viewport_u = vec3(viewport_width, 0.0, 0.0);
    vec3 viewport_v = vec3(0.0, -viewport_height, 0.0);

    // horizontal and vertical delta vectors from pixel to pixel
    // vec3 pixel_delta_u = viewport_u / image_width;

    vec3 viewport_upper_left = camera_center - vec3(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
    // vec3 pixel00_loc = vec3()
    
    
    vec3 pixel_center = vec3(uv.xy, viewport_upper_left.z);
    vec3 ray_direction = pixel_center - camera_center;

    Ray r = Ray(camera_center, ray_direction);

    vec3 pixel_color = ray_color(r);

    color = vec4(pixel_color, 1.0);
}