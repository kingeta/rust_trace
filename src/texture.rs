use super::vector::*;
use super::colour::*;
use image::{GenericImageView};

//const PI: f32 = 3.14159;

pub fn to_angles(v: Vec3) -> (f32, f32) {
    ((v.z()/v.length()).acos(), v.x().atan2(v.y()))
}

pub trait Texture {
    fn value(&self, u: f32, v: f32) -> Colour;
    // I guess maybe u, v are from -1 to 1 or something
}

pub struct TextConstant {
    col: Colour,
}

impl Texture for TextConstant {
    fn value(&self, _: f32, _: f32) -> Colour {
        self.col
    }
}

pub struct TextCheck {
    col1: Colour,
    col2: Colour,
}

impl Texture for TextCheck {
    fn value(&self, u: f32, v: f32) -> Colour{
        let lerp = ((3. * u) % 1.).abs().round() * ((3. * v) % 1.).abs().round();
        lerp * self.col1 + (1.-lerp) * self.col2
    }
}

pub fn text_check(v: Vec3) -> Colour {
    let (t, p) = to_angles(v);
    Colour::white() * ((3. * t) % 1.).abs().round() * ((3. * p) % 1.).abs().round()
}

fn text_image(v: Vec3, img: image::DynamicImage) -> Colour {

    //let mut img = image::open("worldmap.png").unwrap();

    let (t, p) = to_angles(v);
    let x = (t/PI*img.width() as f32).round() as u32;
    let y = (p/(2.*PI)*img.height() as f32).round() as u32;
    
    
    let col = img.get_pixel(x, y);
    //Colour {r: col[0] as f32 / 255., g: col[1] as f32 / 255., b: col[2] as f32 / 255.}
    Colour::new(col[0] as f32 / 255., col[1] as f32 / 255., col[2] as f32 / 255.)
}
