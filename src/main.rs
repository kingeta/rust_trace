use image::{ImageBuffer, Rgb, RgbImage, Pixel, GenericImageView, GenericImage};
//use std::time::{SystemTime, UNIX_EPOCH};
//use std::f32::INFINITY;

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


    // Setting up the camera
    let eye = Vec3::new(0., 0.2, 0.);
    let looking = Vec3::new(0., -0.051, 1.);
    let global_up = Vec3::new(0., 1., 0.);

    
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

    //let mut seed: u32 = SystemTime::now().duration_since(UNIX_EPOCH).ok().unwrap().as_millis() as u32;

    let cam = SimpleCamera {
        fov: PI/3., //0.6435
        position: eye,
        looking: looking,
        global_up: global_up,
    };

    cam.render(shapes, width, height, SAMPLES, "new_result.png".to_string());
}
