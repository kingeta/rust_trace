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

    const SAMPLES: u32 = 8;


    // Setting up the camera
    let eye = Vec3::new(-2., 2., -3.);
    //let looking = Vec3::new(0., -0.8, 1.).normalise();
    let looking = (Vec3::new(0., 0.7, 0.)-eye).normalise();
    let global_up = Vec3::new(0., 1., 0.).normalise();

    
    // Setting up the scene
    // Setting up the spheres

    let floor = Plane {
        point: Vec3::new(0., 0., 0.),
        normal: Vec3::new(0., 1., 0.),
        colour: Colour::new(0.2, 0.4, 0.2),
        material: LAMBERT,
    };


    let orb = Sphere {
        centre: Vec3::new(1.8, 1., 0.),
        radius: 1.,
        texture: |_| {Colour::white()},
        material: GLASS,
    };


    let check = Sphere {
        centre: Vec3::new(0., 0.7, 0.),
        radius: 0.7,
        //texture: |v| { text_check(v) },
        texture: |_| { Colour::white() },
        material: LAMBERT,
    };

    fn sphere_dist(p: Vec3) -> f32 {
        p.length() - 0.7
    }

    const ITERATIONS: i32 = 10;
    const BAILOUT: f32 = 10.;
    const POWER: f32 = 2.;

    fn mand_dist(pos: Vec3) -> f32 {
        let mut z = pos;
        let mut dr = 1.;
        let mut r: f32 = 0.;

        for _ in 0..ITERATIONS {
            r = z.length();
            if r > BAILOUT {break;}

            let mut theta = (z.z()/ r).acos();
            let mut phi = (z.y()).atan2(z.x());
            dr = r.powf(POWER - 1.) * POWER * dr + 1.;

            let zr = r.powf(POWER);
            theta = theta * POWER;
            phi = phi * POWER;

            z = zr * Vec3::new(theta.sin()*phi.cos(), phi.sin() * theta.sin(), theta.cos());
            z += pos;
        }

        0.5 * r.ln() * r/dr
    }

    let check2 = March {
        centre: Vec3::new(0., 0.7, 0.),
        scale: 0.7,
        dist: mand_dist,
        colour: Colour::white(),
        material: LAMBERT,
    };

    let light = Sphere {
        centre: Vec3::new(-1.3, 0.5, 0.),
        radius: 0.5,
        texture: |_| { Colour::new(1., 0.8, 0.) },
        material: LIGHT,
    };

    let cube = AABB {
        min: Vec3::new(-1., -1., -1.),
        max: Vec3::new(1., 1., 1.),
        material: LAMBERT,
        colour: Colour::new(0.8, 0.1, 0.1),
    };

    let trig = Triangle {
        v0: Vec3::new(-1., -1., 0.),
        v1: Vec3::new(0., 1., 0.),
        v2: Vec3::new(1., -1., 0.),
        material: LAMBERT,
        colour: Colour::white(),
    };

    let rect = Rect_XY {
        x0: 3, x1: 5,
        y0: 1, y1: 3,
        k: -2,
        material: LAMBERT,
        colour: Colour::white(),
    }

    //let spheres = [small, big, other, left];
    let shapes: Vec<Box<dyn Object>> = vec![Box::new(floor), Box::new(orb), Box::new(rect), Box::new(light)];

    //let mut seed: u32 = SystemTime::now().duration_since(UNIX_EPOCH).ok().unwrap().as_millis() as u32;

    let cam = SimpleCamera {
        fov: PI/4., //0.6435
        position: eye,
        looking: looking,
        global_up: global_up,
    };

    /*let cam = DOFCamera {
        fov: PI/4., //0.6435
        position: eye,
        looking: looking,
        global_up: global_up,
        aperture: 0.3,
        focus: 3.42,
    };*/

    cam.render(shapes, width, height, SAMPLES, "new_result.png".to_string());
}
