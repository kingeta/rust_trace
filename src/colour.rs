use image::{Rgb, Pixel};


//mod vector;
use super::vector::*;


const GAMMA: f32 = 2.2;
const BRIGHTNESS: f32 = 2.;
//const PI: f32 = 3.14159;


pub fn clamp(val: f32) -> f32 {
    val.min(1.).max(0.)
}

fn gamma_encode(linear: f32) -> f32 {
    linear.powf(1./GAMMA)
}

fn exp(x: f32) -> f32 {
    //return x/4.;
    1. - (-x * BRIGHTNESS).exp()
}

pub type Colour = Vec3;

impl Colour {
    pub fn r(&self) -> f32 {
        self.value[0]
    }
    pub fn g(&self) -> f32 {
        self.value[1]
    }
    pub fn b(&self) -> f32 {
        self.value[2]
    }

    pub fn black() -> Colour {
        Colour::new(0., 0., 0.)
    }

    pub fn white() -> Colour {
        Colour::new(1., 1., 1.)
    }

    pub fn clamp(&self) -> Colour {
        Colour::new(
            clamp(self.value[0]),
            clamp(self.value[1]),
            clamp(self.value[2]),)
    }

    pub fn to_rgb(&self) -> Rgb<u8> {
        Rgb::from_channels(
            (gamma_encode(exp(self.r())) * 255.) as u8,
            (gamma_encode(exp(self.g())) * 255.) as u8,
            (gamma_encode(exp(self.b())) * 255.) as u8,
            0,
        )
    }
}