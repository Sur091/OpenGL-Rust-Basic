#version 450 core


in vec4 vertex_uv;

layout(location=0) out vec4 color;


struct Camera {
    float aspect_ratio, image_width, image_height;
    vec3 center, pixel00_loc, pixel_delta_u, pixel_delta_v;
};

uniform Camera u_camera;

vec2 uv = vertex_uv.xy / 2.0 + 0.5;
float i = uv.x * u_camera.image_width;
float j = (1.0 - uv.y) * u_camera.image_height;

struct Ray {
    vec3 origin;
    vec3 direction;
};

vec3 ray_at(in Ray ray, float t) {
    return ray.origin + t * ray.direction;
}

float random (vec2 st) {
    return fract(sin(dot(st.xy,
                         vec2(12.9898,78.233)))*
        43758.5453123);
}

float random(float x) {
    return fract(sin(x) * 43758.5453123);
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

struct Interval {
    float min, max;
};
// I believe that 65504.0 is the maximum number for a 16-bit float. I didn't choose infinity because I am no sure if the representation changes with the precision.
// Basically, I need to reasearch more. But I don't want to.
const float INFINITY = 65500.0; 
const Interval EMPTY = Interval(INFINITY, -INFINITY);
const Interval UNIVERSE = Interval(-INFINITY, INFINITY);
const int samples_per_pixel = 10;
const float pixel_samples_scale = 1.0 / float(samples_per_pixel);


struct Sphere {
    vec3 center;
    float radius;
};

const int number_of_spheres = 2;

struct HittableList {
    Sphere spheres[2];
};

bool sphere_hit(in Sphere sphere, in Ray r, Interval ray_t, inout HitRecord rec) {
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
    if (root < ray_t.min || ray_t.max < root) {
        root = (h + sqrtd) / a;
        if (root < ray_t.min || ray_t.max < root)
            return false;
    }

    rec.t = root;
    rec.p = ray_at(r, rec.t);
    vec3 outward_normal = (rec.p - sphere.center) / sphere.radius;
    hit_record_set_front_face(rec, r, outward_normal);

    return true;
}

bool hittable_list_hit(inout HittableList hittable_list, in Ray r, Interval ray_t, inout HitRecord rec) {
    HitRecord temp_rec = HitRecord(vec3(0.0), vec3(0.0), 0.0, false);
    bool hit_anything = false;
    float closest_so_far = ray_t.max;

    for (int i = 0; i < number_of_spheres; i++) {
        if (sphere_hit(hittable_list.spheres[i], r, Interval(ray_t.min, closest_so_far), temp_rec)) {
            hit_anything = true;
            closest_so_far = temp_rec.t;
            rec = temp_rec;
        }
    }

    return hit_anything;
}

vec3 ray_color(in Ray r, in HittableList world) {
    HitRecord rec = HitRecord(vec3(0.0), vec3(0.0), 0.0, false);
    bool t = hittable_list_hit(world, r, Interval(0.0, INFINITY), rec);
    if (t) {
        vec3 N = rec.normal;
        return 0.5 * (N + 1.0);
    }
    vec3 unit_direction = normalize(r.direction);
    float a = 0.5 * (unit_direction.y + 1.0);
    return (1.0 - a) * vec3(1.0) + a * vec3(0.5, 0.7, 1.0);
}


Ray get_ray() {
    vec3 pixel_center = u_camera.pixel00_loc + (i * u_camera.pixel_delta_u) + (j * u_camera.pixel_delta_v);
    vec3 ray_direction = pixel_center - u_camera.center;
    return Ray(u_camera.center, ray_direction);
}

vec3 render(in HittableList world) {
    
    vec3 pixel_color = vec3(0.0);
    for (int i =0; i < samples_per_pixel; i++) {
        Ray r = get_ray();
        pixel_color += ray_color(r, world);
    }
    return pixel_color * pixel_samples_scale;
}

void main()
{  
    // World

    HittableList world;
    world.spheres[0] = Sphere(vec3(0.0, 0.0, -1.0), 0.5);
    world.spheres[1] = Sphere(vec3(0.0, -100.5, -1.0), 100.0);

    vec3 pixel_color = render(world);

    color = vec4(pixel_color, 1.0);
}