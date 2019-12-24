use image::{ImageBuffer, Rgb, RgbImage, Pixel, GenericImageView, GenericImage};
//use std::time::{SystemTime, UNIX_EPOCH};
use std::f32::INFINITY;

mod vector;
use vector::*;
mod colour;
use colour::*;
mod material;
use material::*;
mod primitive;
use primitive::*;
mod texture;
use texture::*;
mod render;
use render::*;


//const PI: f32 = 3.14159;

/*
fn get_min(nets: &Vec<f32>) -> f32 {
    return nets.iter().fold(INFINITY, |min, &val| if val < min{ val } else{ min });
    //result
}

fn get_min_index(nets: &Vec<f32>) -> usize {
    let mut i: usize = 0;
    let mut result: usize = 0;

    nets.iter().fold(INFINITY, |mut min, &val| {
        if val < min {
            min = val;
            result = i;}
        i += 1;
        min});

    result
}
*/

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
            let mut hitpos = o + d * dist;

            // New direction
            let newd = (mat.brdf)(norm, d, &mut seed);

            // Offset the hit position a bit by the normal
            // If the generated ray is reflected then bump the normal away from the surface,
            // elif the ray is a refracted ray, bump the normal into the surface
            hitpos += norm * 0.01 * if dot(newd, norm) > 0. {1.} else {-1.};

            let emittance = col * mat.emission;

            let albedo = mat.albedo * if mat.cos {clamp(dot(d, -norm))} else {1.};
            let reflectance = col * trace(hitpos, newd, scene, depth - 1, &mut seed) * albedo;

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


fn main() {    
    // Setting up the final image
    let width: u32;
    let height: u32;

    if false {
        width = 3840 * 2;
        height = 2160 * 2;
    } else {
        width = 640;
        height = 320;    
    }

    const SAMPLES: u32 = 32;

    // Set up the resulting image
    let mut img: RgbImage = ImageBuffer::new(width, height);

    // Setting up the camera
    let eye = Vec3::new(0., 0.2, 0.);
    let looking = Vec3::new(0., -0.051, 1.);
    let global_up = Vec3::new(0., 1., 0.);
    let (side, up) = directions(looking, global_up);

    // Setting up the scene
    // Setting up the spheres

    let small = Sphere {
        centre: Vec3::new(1.5, 0., 6.),
        radius: 1.,
        texture: |_| {Colour::white()},
        material: GLASS,
    };

    /*
    let big = Sphere {
        centre: Vec3::new(0., -10_000 as f32, 12.),
        radius: 10_000 as f32 - 1.,
        //texture: |_| {Colour {r:0.2, g: 0.4, b: 0.2}},
        texture: |_| { Colour::new(0.2, 0.4, 0.2) },
        material: LAMBERT,
    };
    */

    let big = Plane {
        point: Vec3::new(0., -1., 0.),
        normal: Vec3::new(0., 1., 0.),
        colour: Colour::new(0.2, 0.4, 0.2),
        material: LAMBERT,
    };

    let other = Sphere {
        centre: Vec3::new(1.6, -0.3, 9.5),
        radius: 0.7,
        texture: |v| { text_check(v) },
        material: LAMBERT,
    };

    let left = Sphere {
        centre: Vec3::new(-2., -0.5, 6.5),
        radius: 0.5,
        texture: |_| { Colour::new(1., 0.8, 0.) },
        material: LIGHT, //METAL
    };

    /*
    let cube = AABB {
        min: Vec3::new(-0.5, 0., 5.),
        max: Vec3::new(0.5, 1., 6.),
        material: LAMBERT,
        colour: Colour::new(0.8, 0.1, 0.1),
    };
    */

    //let spheres = [small, big, other, left];
    let shapes: Vec<Box<dyn Object>> = vec![Box::new(small), Box::new(big), Box::new(other), Box::new(left)];

    // Setting up variables which are used later
    let mut direction: Vec3;

    let mut col: Colour;
    //let mut seed: u32 = SystemTime::now().duration_since(UNIX_EPOCH).ok().unwrap().as_millis() as u32;
    let mut seed: u32 = 4839;

    let mut u: f32;
    let mut v: f32;

    for (x, y, pixel) in img.enumerate_pixels_mut() {
        // Initialise the colour
        col = Colour::black();
        
        for _ in 0..SAMPLES {
            u = ((2.*x as f32)/((width-1) as f32) - 1.)*(width as f32)/(height as f32);
            v = (-2.*y as f32)/((height-1) as f32) + 1.;

            // Maybe these constants are off; unknown
            u += (random_float(&mut seed) - 0.5)/(height as f32 - 1.)*2.;
            v += (random_float(&mut seed) - 0.5)/(height as f32 - 1.)*2.;
            //direction = vec3(u, v, 3.).normalize();
            direction = (3. * looking + u * side + v * up).normalise();

            //col += trace(eye, direction, &spheres[..], 4, &mut seed) * (1./SAMPLES as f32);
            col += trace(eye, direction, &shapes, 4, &mut seed) * (1./SAMPLES as f32);
        }

        *pixel = col.clamp().to_rgb();

    }

    img.save("result.png").unwrap();
    println!("All done");

    let cam = SimpleCamera {fov: 1.};

    cam.render(eye, looking, global_up, shapes, width, height, SAMPLES, "result2.png".to_string());
}
