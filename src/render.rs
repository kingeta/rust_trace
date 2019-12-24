use image::{ImageBuffer, RgbImage};
use super::vector::*;
use super::colour::*;
use super::primitive::*;
use super::material::random_float;


//texture, material (?)

//// Define a scene, with a background and the shapes in it
/*pub struct Scene (
    pub dyn Object,
    pub fn(Vec3) -> Colour,
);*/

fn trace<T: Object>(o: Vec3, d: Vec3, scene: &T, depth: u32, mut seed: &mut u32) -> Colour {
    //// Raytrace a whole scene
    if depth == 0 {
        // Return white if the raytracing depth is reached
        // Originally this returned error red; now it returns white
        return Colour::white();
    }

    let hit = scene.intersect(o, d);

    match hit {
        None => {return bg_colour(d)},
        Some(Hit(dist, hitpos, mat, col, norm)) => {

            // Get info about the shape & render or something
            //let mut hitpos = o + d * dist;

            // New direction
            let newd = (mat.brdf)(norm, d, &mut seed);

            // Offset the hit position a bit by the normal
            // If the generated ray is reflected then bump the normal away from the surface,
            // elif the ray is a refracted ray, bump the normal into the surface
            //hitpos += norm * 0.01 * if dot(newd, norm) > 0. {1.} else {-1.};

            let emittance = col * mat.emission;

            let albedo = mat.albedo * if mat.cos {clamp(dot(d, -norm))} else {1.};
            let reflectance = col * trace(hitpos + norm * 0.01 * if dot(newd, norm) > 0. {1.} else {-1.}, newd, scene, depth - 1, &mut seed) * albedo;

            return (emittance + reflectance) * (1./(PI * mat.prob));
        }
    }

}


fn directions(looking: Vec3, global_up: Vec3) -> (Vec3, Vec3) {
    // Convert (unit) vectors for viewing direction and global up into orthonormal basis for camera
    // 'local' up points approx towards global up but perp to looking
    // Only returns the unknown vectors; program already has looking
    //let side = global_up.cross(looking).normalize();
    let side = cross(global_up, looking).normalise();
    //return (side, looking.cross(side));
    return (side, cross(looking, side));
}


fn bg_colour(d: Vec3) -> Colour {
    //// A function which gives a sky colour based on a direction

    // Sun colour
    let sundirec = Vec3::new(-1., 1., -1.).normalise();
    let sunlight = Colour::white() * clamp(dot(sundirec, d) + 0.03).powf(300.);

    // Lerp between blue and white vertically
    //let val = ((1. + v.y)/2.).powf(1.5);
    let val = ((1. + d.y())/2.).powf(1.5);
    let sky = (Colour::new(0.45, 0.68, 0.87) * (1.-val) + Colour::white() *  val) * 0.4;

    return sunlight * 1. + sky;
}


//// Whether something can render
pub trait Render {
    fn render(&self, scene: Vec<Box<dyn Object>>, width: u32, height: u32, samples: u32, filename: String);
}

//// A pinhole camera
pub struct SimpleCamera {
    // x-direction fov
    pub fov: f32,
    pub position: Vec3,
    pub looking: Vec3,
    pub global_up: Vec3,
}

impl Render for SimpleCamera {
    fn render(&self, scene: Vec<Box<dyn Object>>, width: u32, height: u32, samples: u32, filename: String) {
        // The resulting image
        let mut finalimg: RgbImage = ImageBuffer::new(width, height);
        // The other 2 vectors for an orthonormal basis
        let (side, up) = directions(self.looking, self.global_up);

        let mut direction: Vec3;
        let mut u: f32;
        let mut v: f32;

        let h = 1./(self.fov/2.).tan();

        // The resulting colour at a point
        let mut col: Colour;
        // The seed
        let mut seed: u32 = 4839;
    
        for (x, y, pixel) in finalimg.enumerate_pixels_mut() {
            // Initialise the colour
            col = Colour::black();
            
            for _ in 0..samples {
                u = ((2.*x as f32)/((width-1) as f32) - 1.)*(width as f32)/(height as f32);
                v = (-2.*y as f32)/((height-1) as f32) + 1.;
    
                // Maybe these constants are off; unknown
                u += (random_float(&mut seed) - 0.5)/(height as f32 - 1.)*2.;
                v += (random_float(&mut seed) - 0.5)/(height as f32 - 1.)*2.;
                // originally h = 3.
                direction = (h * self.looking + u * side + v * up).normalise();
    
                col += trace(self.position, direction, &scene, 4, &mut seed) //* (1./SAMPLES as f32);
            }
    
            *pixel = (col / samples as f32).clamp().to_rgb();
    
        }
    
    finalimg.save(filename).unwrap();
    println!("All done");    
    }
}