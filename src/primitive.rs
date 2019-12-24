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

pub struct AABB {
    pub min: Vec3,
    pub max: Vec3,
    pub colour: Colour,
    pub material: Material,
}


impl Object for AABB {
    fn intersect(&self, o: Vec3, d: Vec3) -> Option<Hit> {
        let mut tmin = 0.;
        let mut tmax = 1_000.;

        for a in 0..3 {
            let inv_d = 1./d.value[a];
            let t0 = (self.min.value[a] - o.value[a]) * inv_d;
            let t1 = (self.max.value[a] - o.value[a]) * inv_d;
            if inv_d < 0. {
                let (t0, t1) = (t1, t0);
            }

            tmin = if t0 > tmin {t0} else {tmin};
            tmax = if t1 < tmax {t1} else {tmax};

            if tmax < tmin {
                return None;
            }

        }

        let t = if tmin > 0. {tmin} else {tmax};

        Some(Hit(t, o + d * t, self.material, self.colour, Vec3::new(1., 0., 0.)))
    }
}