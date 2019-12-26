use std::ops::{Add, Sub, Mul, Div, AddAssign, Neg};
//use std::ops::*;
//? use std::f32;

pub const PI: f32 = 3.14159;

// Vector construct
#[derive(Copy, Clone)]
pub struct Vec3 {
    pub value: [f32; 3]
}

pub fn dot(u: Vec3, v: Vec3) -> f32 {
    v.value[0]*u.value[0] + v.value[1]*u.value[1] + v.value[2]*u.value[2]
}

pub fn cross(u: Vec3, v: Vec3) -> Vec3 {
    Vec3::new(
            u.value[1]*v.value[2] - u.value[2]*v.value[1],
            u.value[2]*v.value[0] - u.value[0]*v.value[2],
            u.value[0]*v.value[1] - u.value[1]*v.value[0])
}

pub fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    // Reflect v in n, which should be normalised
    v - 2.*dot(v, n)*n

}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Vec3 {
        Vec3 {value: [
            self.value[0] + other.value[0],
            self.value[1] + other.value[1],
            self.value[2] + other.value[2]
            ]}
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Vec3 {
        Vec3 {value: [
            self.value[0] - other.value[0],
            self.value[1] - other.value[1],
            self.value[2] - other.value[2]
        ]}
    }
}

impl Mul for Vec3 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Vec3{
        Vec3 {value: [
            self.value[0] * other.value[0],
            self.value[1] * other.value[1],
            self.value[2] * other.value[2]
        ]}
    }
}

impl Mul<f32> for Vec3 {
    type Output = Vec3;

    fn mul(self, t: f32) -> Vec3 {
        Vec3 {value: [
            self.value[0]*t,
            self.value[1]*t,
            self.value[2]*t
        ]}
    }
}

impl Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, v: Vec3) -> Vec3 {
        v * self
    }
}

impl Div<f32> for Vec3 {
    type Output = Vec3;

    fn div(self, t: f32) -> Vec3 {
        Vec3::new(self.value[0]/t, self.value[1]/t, self.value[2]/t)
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Vec3) {
        self.value[0] += other.value[0];
        self.value[1] += other.value[1];
        self.value[2] += other.value[2];
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        Vec3 {value: [-self.value[0], -self.value[1], -self.value[2]]}
    }
}

// Some convenience functions
impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3 {value: [x, y, z]}
    }

    pub fn x(&self) -> f32 {
        self.value[0]
    }
    pub fn y(&self) -> f32 {
        self.value[1]
    }
    pub fn z(&self) -> f32 {
        self.value[2]
    }

    /*pub fn length_squared(&self) -> f32 {
        self.value[0]*self.value[0] + self.value[1]*self.value[1] + self.value[2]*self.value[2]
    }

    pub fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }*/

    pub fn length_squared(&self) -> f32 {
        dot(*self, *self)
    }

    pub fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }

    pub fn normalise(self) -> Vec3 {
        self / self.length()
    }
}