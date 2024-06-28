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

struct HitRecord {
    vec3 p;
    vec3 normal;
    float t;
    bool front_face;
};

void hit_record_set_front_face(inout HitRecord hit_record, in Ray r, in vec3 outward_normal) {
    hit_record.front_face = dot(r.direction, outward_normal) < 0.0;
    hit_record.normal = hit_record.front_face ? outward_normal: -outward_normal;
}
`
struct Sphere {
    vec3 center;
    float radius;
};

const int number_of_spheres = 2;

struct HittableList {
    Sphere spheres[2];
};

bool sphere_hit(in Sphere sphere, in Ray r, float ray_tmin, float ray_tmax, inout HitRecord rec) {
    vec3 oc = sphere.center - r.origin;
    float a = dot(r.direction, r.direction);
    float h = dot(r.direction, oc);
    float c = dot(oc, oc) - sphere.radius * sphere.radius;

    float discriminant = h*h - a*c;
    if (discriminant < 0) {
        return false;
    }
    float sqrtd = sqrt(discriminant);

    // Find the nearest root that lies in the acceptable range.
    float root = (h - sqrtd) / a;
    if (root <= ray_tmin || ray_tmax <= root) {
        root = (h + sqrtd) / a;
        if (root <= ray_tmin || ray_tmax <= root)
            return false;
    }

    rec.t = root;
    rec.p = ray_at(r, rec.t);
    vec3 outward_normal = (rec.p - sphere.center) / sphere.radius;
    hit_record_set_front_face(rec, r, outward_normal);

    return true;
}

bool hittable_list_hit(inout HittableList hittable_list, in Ray r, float ray_tmin, float ray_tmax, inout HitRecord rec) {
    HitRecord temp_rec = HitRecord(vec3(0.0), vec3(0.0), 0.0, false);
    bool hit_anything = false;
    float closest_so_far = ray_tmax;

    for (int i = 0; i < number_of_spheres; i++) {
        if (sphere_hit(hittable_list.spheres[i], r, ray_tmin, closest_so_far, temp_rec)) {
            hit_anything = true;
            closest_so_far = temp_rec.t;
            rec = temp_rec;
        }
    }

    return hit_anything;
}

vec3 ray_color(in Ray r, in HittableList world) {
    HitRecord rec = HitRecord(vec3(0.0), vec3(0.0), 0.0, false);
    bool t = hittable_list_hit(world, r, 0.0, 99999.0, rec);
    if (t) {
        vec3 N = rec.normal;
        return 0.5 * (N + 1.0);
    }
    vec3 unit_direction = normalize(r.direction);
    float a = 0.5 * (unit_direction.y + 1.0);
    return (1.0 - a) * vec3(1.0) + a * vec3(0.5, 0.7, 1.0);
}

void main()
{   
    vec2 uv = vec2(vertex_uv.x * u_aspect_ratio, vertex_uv.y);

    // World

    HittableList world;
    world.spheres[0] = Sphere(vec3(0.0, 0.0, -1.0), 0.5);
    world.spheres[1] = Sphere(vec3(0.0, -100.5, -1.0), 100.0);

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

    vec3 pixel_color = ray_color(r, world);

    color = vec4(pixel_color, 1.0);
}