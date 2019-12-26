use std::f32::INFINITY;
use super::vector::*;
use super::material::*;
use super::colour::*;
use super::texture::*;

//////////////////////////////////
//// Holds information about a hit
//////////////////////////////////
#[derive(Copy, Clone)]
pub struct Hit (
    pub f32, // Distance
    pub Vec3, // Point
    pub Material, // Material
    pub Colour, // Colour (at that point)
    pub Vec3, // Normal
);

///////////////////////////////////////////////
//// An object (only has an intersection (???))
///////////////////////////////////////////////
pub trait Object {
    fn intersect(&self, o: Vec3, d: Vec3) -> Option<Hit>;
}

/////////////
//// A Sphere
/////////////
/*pub struct Sphere {
    pub centre: Vec3,
    pub radius: f32,
    pub texture: fn (Vec3) -> Colour,
    pub material: Material,
}*/
pub struct Sphere {
    pub centre: Vec3,
    pub radius: f32,
    pub material: Material,
    pub texture: fn(Vec3) -> Colour,
}

impl Object for Sphere {
    fn intersect(&self, o: Vec3, d: Vec3) -> Option<Hit> {
        let a = dot(d, d);
        let to = o - self.centre;
        let b = 2. * dot(d, to);
        let c = dot(to, to) - self.radius*self.radius;

        let d2 = b*b-4.*a*c;

        if d2 < 0. {
            return None;
        }
        let mut res = (-b - d2.sqrt())/(2.*a);
        if res < 0. {
            let res2 = (-b + d2.sqrt())/(2.*a);
            if res2 < 0. {
                return None;
            }
            res = res2
            //return Some(Hit(res2, self.material, (self.texture)(o+res2*d - self.centre), self.normal(o+res2*d)));
        }

        let hitpos = o + res * d;

        Some(Hit(res, hitpos, self.material, (self.texture)(hitpos - self.centre), self.normal(hitpos)))
    }

}

impl Sphere {
    fn normal(&self, p: Vec3) -> Vec3 {
        (p - self.centre)/self.radius
    }
}

impl Object for Vec<Box<dyn Object>> {
    //// This is clever; make a vector of boxed Objects an Object
    //// so that they can trivially be intersected
    fn intersect(&self, o: Vec3, d: Vec3) -> Option<Hit> {
        let mut hit: Option<Hit> = None;

        for object in self.iter() {
            if let Some(candidate_hit) = object.intersect(o, d) {
                match hit {
                    None => hit = Some(candidate_hit),
                    Some(prev) => if candidate_hit.0 < prev.0 {
                        hit = Some(candidate_hit);
                    }
                }
            }
        }

        hit
    }
}

////////////////////////////////////////////////////////
//// A general plane (currently without texture support)
////////////////////////////////////////////////////////
pub struct Plane {
    pub point: Vec3,
    pub normal: Vec3,
    pub colour: Colour,
    pub material: Material,
}

impl Object for Plane {
    fn intersect(&self, o: Vec3, d: Vec3) -> Option<Hit> {
        let angle = dot(d, self.normal);
        
        if angle.abs() < 0.001 {
            return None;
        }

        let t =  dot(self.point - o, self.normal)/angle;

        if t <= 0. {
            return None
        }

        Some(Hit(t, o+t*d, self.material, self.colour, self.normal))
    }
}

pub struct Rect_XY {
    pub x0: f32,
    pub x1: f32,
    pub y0: f32,
    pub y1: f32,
    pub k: f32,
    pub colour: Colour,
    pub material: Material,
}

impl Object for Rect_XY {
    fn intersect(&self, o: Vec3, d: Vec3) -> Option<Hit> {
        let t = (self.k - o.z()) / d.z();
        let x = r.x() + t * d.x();
        let y = r.y() + t * d.y();

        if x < x0 || x > x1 || y < y0 || y > y1 {
            return None;
        }

        Some(Hit(t, o + t * d, self.material, self.colour, Vec3::new(0., 0., 1.)))
    }
}

pub struct Triangle {
    pub v0: Vec3,
    pub v1: Vec3,
    pub v2: Vec3,
    pub colour: Colour,
    pub material: Material,
}

impl Object for Triangle {
    fn intersect(&self, o: Vec3, d: Vec3) -> Option<Hit> {
        let v0v1 = self.v1 - self.v0;
        let v0v2 = self.v2 - self.v0;

        let norm = cross(v0v1, v0v2);
        let area2 = norm.length();

        let norm_direction = dot(norm, d);

        if norm_direction.abs() < 0.0001 {
            return None;
        }

        let d2 = dot(norm, self.v0);
        let t: f32 = (dot(norm, o) + d2)/norm_direction;

        if t < 0. {
            return None;
        }

        let p = o + d * t; // Plane intersection position

        let edge0 = v0v1;
        let vp0 = p - self.v0;
        if dot(norm, cross(edge0, vp0)) < 0. {
            return None;
        }

        let edge1 = self.v2 - self.v1;
        let vp1 = p - self.v1;
        if dot(norm, cross(edge1, vp1)) < 0. {
            return None;
        }

        let edge2 = self.v0 - self.v2;
        let vp2 = p - self.v2;
        if dot(norm, cross(edge2, vp2)) < 0. {
            return None;
        }


        Some(Hit(t, p, self.material, self.colour, norm))
    }
}

pub struct AABB {
    pub min: Vec3,
    pub max: Vec3,
    pub colour: Colour,
    pub material: Material,
}


impl Object for AABB {
    fn intersect(&self, o: Vec3, d: Vec3) -> Option<Hit> {
        let mut norm = Vec3::new(0., 0., 0.);
        let mut t_near = -INFINITY;
        let mut t_far = INFINITY;
        let mut t_1 = Vec3::new(0., 0., 0.);
        let mut t_2 = Vec3::new(0., 0., 0.);

        for i in 0..3 {
            if d.value[i] == 0. {
                if o.value[i] < self.min.value[i] || o.value[i] > self.max.value[i] {
                    return None;
                }
            } else {
                // Ray not parallel
                t_1.value[i] = (self.min.value[i] - o.value[i]) / d.value[i];
                t_2.value[i] = (self.max.value[i] - o.value[i]) / d.value[i];

                if t_1.value[i]>t_2.value[i] {
                    let (t_2, t_1) = (t_1, t_2);
                }

                if t_1.value[i] > t_near {
                    t_near = t_1.value[i];
                }

                if t_2.value[i] < t_far {
                    t_far = t_2.value[i];
                }

                if t_near > t_far || t_far < 0. {
                    return None;
                }
            }
        }

        let t = if t_near > 0. {t_near} else {t_far};
        Some(Hit(t, o + t * d, self.material, self.colour, Vec3::new(1., 1., 1.)))
    }
}

const MARCHDEPTH: u32 = 100;
const EPSILON: f32 = 0.001;
const MARCHMAX: f32 = 1_000.;

pub struct March {
    pub centre: Vec3,
    pub scale: f32,
    pub dist: fn(Vec3) -> f32,
    pub material: Material,
    pub colour: Colour,
}

impl March {
    pub fn normal(&self, p: Vec3) -> Vec3 {
        let x = Vec3::new(EPSILON, 0., 0.);
        let y = Vec3::new(0., EPSILON, 0.);
        let z = Vec3::new(0., 0., EPSILON);

        return Vec3::new(
            ((self.dist)(p + x - self.centre) - (self.dist)(p - x - self.centre)) / (2. * EPSILON),
            ((self.dist)(p + y - self.centre) - (self.dist)(p - y - self.centre)) / (2. * EPSILON),
            ((self.dist)(p + z - self.centre) - (self.dist)(p - z - self.centre)) / (2. * EPSILON),
        )
    }
}

impl Object for March {
    fn intersect(&self, o: Vec3, d: Vec3) -> Option<Hit> {
        let mut p = o;
        let mut dist: f32;

        for _ in 0..MARCHDEPTH {
            dist = (self.dist)(p - self.centre);
            p += dist * d;

            if dist < EPSILON {
                return Some(Hit(dist, p, self.material, self.colour, self.normal(p - self.centre)));
            }

            if dist > MARCHMAX {
                return None;
            }
        }
        None
    }
}