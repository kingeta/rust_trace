use super::vector::*;

pub fn random_float(seed: &mut u32) -> f32 {
    // Random float in [0, 1)
    let mut x = *seed;
    x ^= x >> 13;
    x ^= x << 17;
    x ^= x >> 5;
    *seed = x;
    let float_bits = (x & 0x007FFFFF) | 0x3F800000;
    let float: f32 = unsafe { ::core::mem::transmute(float_bits) };
    return float - 1.0;
}


#[derive(Copy, Clone)]
pub struct Material {
    pub albedo: f32,
    pub brdf: fn(Vec3, Vec3, &mut u32) -> Vec3,
    pub prob: f32,
    pub cos: bool, // Is albedo via cos required (n.l term or something)
    // Maybe I'll put colour in here, maybe not. Not sure what else is needed
    pub emission: f32,
}


fn refract(v: Vec3, n: Vec3, ratio: f32) -> Vec3 {
    // Refract an incoming vector in a normal with the given ratio (n1/n2 from Snell's law)
    // v, n normalised vectors
    
    let dt = dot(v, n);

    let discriminant = 1. - ratio*ratio*(1.-dt*dt);

    if discriminant > 0. {
        // Refract
        let newd = (v-n*dt) * ratio - n * discriminant.sqrt();
        //println!("{}", dot(newd, n));
        return newd;
    } else {
        // Reflect
        //return v - 2.*dot(v, n)*n;
        return reflect(v, n);
    }
}

fn schlick(cos: f32, ratio: f32) -> f32 {
    let r0 = (1. - ratio)/(1. + ratio).powf(2.);
    r0 + (1.-r0)*(1.-cos).powf(5.)
}


fn brdf_mirror(n: Vec3, d: Vec3, _: &mut u32) -> Vec3 {
    // BRDF for a mirror (every ray is reflected)
    d - 2.*dot(n, d)*n
}

fn brdf_glass(n: Vec3, d: Vec3, seed: &mut u32) -> Vec3 {
    // BRDF for glass; some rays reflected, others refracted
    let cos = dot(d, -n); // Cosine of angle of incoming
    let refr = 0.7; // Refractive index
    
    // ... dot(n, d) < 0 ...

    let rand = random_float(seed);

    // && random_float(seed) > schlick(cos, refr) 
    if cos > 0. {
        // Ray comes from outside
        if rand > schlick(cos, refr) {
            // Normally: refract
            return refract(d, n, refr);            
        } else {
            // If close to the edges: possibly reflect
            return reflect(d, n);
        }
    } else {
        // Ray from inside
        if rand > schlick(-cos * refr, refr) {
            // Normally: refract (with higher prob for some reason I think?)
            return refract(d, n * cos.signum(), 1./refr);
        } else {
            // If close to the edges: again maybe reflect
            return reflect(d, n);
        }
    }
}

fn brdf_metal(n: Vec3, d: Vec3, seed: &mut u32) -> Vec3 {
    // Metal sort of thing
    return(d-2.*dot(n, d)*n + 0.4 * brdf_lambert(n, d, seed)).normalise()
}

fn brdf_lambert(n: Vec3, _: Vec3, seed: &mut u32) -> Vec3 {
    // Return a cosine weighted ray, centered around
    // the normal n
    // Ignore the direction of the incoming ray
    let azmith = random_float(seed) * PI * 2.;
    let y = random_float(seed);
    let sin_elevation = (1. - y*y).sqrt();
    let rand = Vec3::new(sin_elevation * azmith.cos(), y, sin_elevation * azmith.sin());

    let nt: Vec3;
    let nb: Vec3;

    if n.x().abs() > n.y().abs() {
        //nt = vec3(n.z, 0., -n.x).normalize();
        nt = Vec3::new(n.z(), 0., -n.x()).normalise();
    } else {
        //nt = vec3(0., -n.z, n.y).normalize();
        nt = Vec3::new(0., -n.z(), n.y()).normalise();
    }

    //nb = n.cross(nt);
    nb = cross(n, nt);

    Vec3::new(
        rand.x() * nb.x() + rand.y() * n.x() + rand.z() * nt.x(),
        rand.x() * nb.y() + rand.y() * n.y() + rand.z() * nt.y(),
        rand.x() * nb.z() + rand.y() * n.z() + rand.z() * nt.z(),
    )
}

pub const LAMBERT: Material = Material {
    albedo: 0.9,
    brdf: brdf_lambert,
    prob: 1./PI, 
    cos: false,
    emission: 0.,
};

pub const MIRROR: Material = Material {
    albedo: 1.,
    brdf: brdf_mirror,
    prob: 1./PI,
    cos: false,
    emission: 0.,
};

pub const LIGHT: Material = Material {
    albedo: 1.,
    brdf: brdf_lambert,
    prob: 1./PI,
    cos: false,
    emission: 0.99,
};

pub const GLASS: Material = Material {
    albedo: 1.,
    brdf: brdf_glass,
    prob: 1./PI,
    cos: false,
    emission: 0.,
};

pub const METAL: Material = Material {
    albedo: 0.9,
    brdf: brdf_metal,
    prob: 1./PI,
    cos: true,
    emission: 0.,
};