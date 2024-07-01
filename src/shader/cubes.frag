#version 450 core


in vec4 vertex_uv;

layout(location=0) out vec4 color;


struct Camera {
    float aspect_ratio, image_width, image_height;
    vec3 center, pixel00_loc, pixel_delta_u, pixel_delta_v;
};

uniform Camera u_camera;
uniform float u_time;

vec2 uv = vertex_uv.xy / 2.0 + 0.5;
float i = uv.x * u_camera.image_width;
float j = (1.0 - uv.y) * u_camera.image_height;

const float PI = 3.141593;

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

float seed = 2334.3594;
float random(float x) {
    seed = fract(sin(seed + u_time + x) * 43758.5453123);
    return seed;
}


vec3 random_unit_vector(vec2 rand) {
    rand = vec2(random(rand.x), random(rand.y)) * 2.0 - 1.0;
    float ang1 = (rand.x + 1.0) * PI; // [-1..1) -> [0..2*PI)
    float u = rand.y; // [-1..1), cos and acos(2v-1) cancel each other out, so we arrive at [-1..1)
    float u2 = u * u;
    float sqrt1MinusU2 = sqrt(1.0 - u2);
    float x = sqrt1MinusU2 * cos(ang1);
    float y = sqrt1MinusU2 * sin(ang1);
    float z = u;
    return vec3(x, y, z);
}

vec3 random_sphere_point(vec2 rand) {
    rand = vec2(random(random(rand.x)), random(random(rand.y))) * 2.0 - 1.0;
    float angle1 = (rand.x + 1.0) * PI;
    float u = rand.y;
    float sqrt1_minus_u2 = sqrt(1.0 - u * u);
    float x = sqrt1_minus_u2 * cos(angle1);
    float y = sqrt1_minus_u2 * sin(angle1);
    return vec3(x, y, u);
}

vec3 random_on_hemisphere(float seed, vec3 normal) {
    vec3 on_unit_sphere = random_sphere_point(vec2(i*seed, j*seed));
    if (dot(on_unit_sphere, normal) > 0.0) {
        return on_unit_sphere;
    } 
    return -on_unit_sphere;
}

// Lambertian = Material.albedo.w < 0.001
// Metal = Material.albedo.w > 0.001
// Dielectric = Material.albedo.x > 1.001
// Note check for Dielectric first.
struct Material {
    vec4 albedo;
};


struct HitRecord {
    vec3 p;
    vec3 normal;
    float t;
    bool front_face;
    Material material;
};

bool near_zero(vec3 point) {
    float delta = 0.001;
    vec3 positive_point = abs(point);
    return (positive_point.x < delta && positive_point.y < delta && positive_point.z < delta);
}

// Lambertian = Material.albedo.w < 0.001
// Metal = Material.albedo.w > 0.001
// Dielectric = Material.albedo.x > 1.001
// Note check for Dielectric first.

bool material_scatter(inout Ray r_in, in HitRecord rec, out vec3 attenuation, out Ray scattered, float seed) {
    // Condition for dielectric
    if (rec.material.albedo.x > 1.001) {
        return false;
    }

    // Condition for Lambertian
    if (rec.material.albedo.w < -0.5) {
        vec3 scatter_direction = rec.normal + random_sphere_point(vec2(seed + u_time, seed * seed + u_time));

        if (near_zero(scatter_direction)) {
            scatter_direction = rec.normal;
        }
        scattered = Ray(rec.p, scatter_direction);
        attenuation = rec.material.albedo.xyz;
        return true;
    }

    // Condition for Metal
    vec3 reflected = reflect(r_in.direction, rec.normal);
    reflected = normalize(reflected) + (rec.material.albedo.w * random_sphere_point(vec2(seed + u_time, seed * seed + u_time)));
    scattered = Ray(rec.p, reflected);
    attenuation = rec.material.albedo.xyz;
    return dot(scattered.direction, rec.normal) > 0.0;
}


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
const int samples_per_pixel = 500;
const int max_depth = 50;
const float pixel_samples_scale = 1.0 / float(samples_per_pixel);


struct Sphere {
    vec3 center;
    float radius;
    Material mat;
};

const int number_of_spheres = 4;

struct HittableList {
    Sphere spheres[number_of_spheres];
};

const Material material_ground = Material(vec4(0.8, 0.8, 0.0, -1.0));
const Material material_center = Material(vec4(0.1, 0.2, 0.5, -1.0));
const Material material_left   = Material(vec4(0.8, 0.8, 0.8, 0.3));
const Material material_right  = Material(vec4(0.8, 0.6, 0.2, 1.0));
const HittableList world = HittableList(
    Sphere[number_of_spheres](
        Sphere(vec3( 0.0, -100.5, -1.0), 100.0, material_ground),
        Sphere(vec3( 0.0,    0.0, -1.2),   0.5, material_center),
        Sphere(vec3(-1.0,    0.0, -1.0),   0.5, material_left),
        Sphere(vec3( 1.0,    0.0, -1.0),   0.5, material_right)
    )
);

bool sphere_hit(in Sphere sphere, in Ray r, Interval ray_t, out HitRecord rec) {
    vec3 oc = sphere.center - r.origin;
    float a = dot(r.direction, r.direction);
    float h = dot(r.direction, oc);
    float c = dot(oc, oc) - sphere.radius * sphere.radius;

    float discriminant = h*h - a*c;
    if (discriminant < 0.0) {
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
    rec.material = sphere.mat;
    vec3 outward_normal = (rec.p - sphere.center) / sphere.radius;
    hit_record_set_front_face(rec, r, outward_normal);

    return true;
}

bool world_hit(in Ray r, Interval ray_t, out HitRecord rec) {
    HitRecord temp_rec;
    bool hit_anything = false;
    float closest_so_far = ray_t.max;

    for (int i = 0; i < number_of_spheres; i++) {
        if (sphere_hit(world.spheres[i], r, Interval(ray_t.min, closest_so_far), temp_rec)) {
            hit_anything = true;
            closest_so_far = temp_rec.t;
            rec = temp_rec;
        }
    }

    return hit_anything;
}

vec3 ray_color(in Ray r) {
    HitRecord rec;
    vec3 hit = vec3(1.0);
    for (int k = 0; k < max_depth; k++) {
        if (world_hit(r, Interval(0.001, INFINITY), rec)) {
            Ray scattered;
            vec3 attenuation;
            if (material_scatter(r, rec, attenuation, scattered, float(k) * u_time / float(max_depth) * 0.1)) {
                hit *= attenuation;
            } else {
                hit *= vec3(0.0);
            }
            r = scattered;
        } else {
            break;
        }
    }
    vec3 unit_direction = normalize(r.direction);
    float a = 0.5 * (unit_direction.y + 1.0);
    vec3 sky_color = vec3(0.5, 0.7, 1.0);
    vec3 color = (1.0 - a) * vec3(1.0) + a * sky_color;
    return  color * hit;
}

vec3 sample_square(float seed) {
    seed = random(seed);
    return vec3(random(uv.x + seed) - 0.5, random(uv.y + seed * seed) - 0.5, 0.0);
}

Ray get_ray(float seed) {
    vec3 offset = sample_square(seed);
    vec3 pixel_sample = u_camera.pixel00_loc
                        + (i + offset.x) * u_camera.pixel_delta_u
                        + (j + offset.y) * u_camera.pixel_delta_v;
    vec3 ray_direction = pixel_sample - u_camera.center;

    return Ray(
        u_camera.center,
        ray_direction
    );
}

vec3 render(in HittableList world) {
    
    vec3 pixel_color = vec3(0.0);
    for (int k = 0; k < samples_per_pixel; k++) {
        Ray r = get_ray(float(k)+j);
        pixel_color += ray_color(r);
    }
    return pixel_color * pixel_samples_scale;
}

void main()
{  
    // World

    // world.spheres(Sphere(vec3(0.0, 0.0, -1.0), 0.5), Sphere(vec3(0.0, -100.5, -1.0), 100.0));


    vec3 pixel_color = render(world);

    color = vec4(sqrt(pixel_color), 1.0);
}