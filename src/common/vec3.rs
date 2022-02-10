use std::{ops::{Add, Div, Mul, Neg, Sub}, io::{self, Write}};

#[derive(Clone, Copy)]
pub struct Vec3(pub f64, pub f64, pub f64);
pub type Point3 = Vec3;
pub type Color = Vec3;

macro_rules! ops_impl_for {
    ($op:ty => $block:tt, $t:ty) => {
        impl $op for $t $block
    };
    ($op:ty => $block:tt, $t:ty, $($ts:ty),+) => {
        impl $op for $t $block
        ops_impl_for!($op => $block, $($ts),* );
    }
}

impl Vec3 {
    pub fn new(e0: f64, e1: f64, e2: f64) -> Self {
        Self(e0, e1, e2)
    }

    pub fn zeros() -> Self {
        Self(0.0, 0.0, 0.0)
    }

    pub fn write_color<W: Write>(&self, mut image: W, samples_per_pixel: u32) -> io::Result<()> {
        let mut r = self.0;
        let mut g = self.1;
        let mut b = self.2;

        let scale = 1.0 / samples_per_pixel as f64;
        r *= scale;
        g *= scale;
        b *= scale;

        write!(
            image,
            "{} {} {}\n",
            (r.clamp(0.0, 0.999) * 256.0) as u32,
            (g.clamp(0.0, 0.999) * 256.0) as u32,
            (b.clamp(0.0, 0.999) * 256.0) as u32
        )?;
        Ok(())
    }
    /*

    fn x(&self) -> f64 {
        self.0
    }
    fn z(&self) -> f64 {
        self.2
    }
    */
    pub fn y(&self) -> f64 {
        self.1
    }

    pub fn length_squared(&self) -> f64 {
        self.0 * self.0 + self.1 * self.1 + self.2 * self.2
    }

    fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn as_unit(&self) -> Self {
        self / self.length()
    }

    pub fn dot(&self, rhs: &Self) -> f64 {
        self.0 * rhs.0 + self.1 * rhs.1 + self.2 * rhs.2
    }
}

ops_impl_for!(Neg => {
    type Output = Vec3;
    fn neg(self) -> Self::Output {
        Vec3(-self.0, -self.1, -self.2)
    }
}, Vec3, &Vec3);

ops_impl_for!(Mul<f64> => {
    type Output = Vec3;
    fn mul(self, rhs: f64) -> Self::Output {
        Vec3(self.0 * rhs, self.1 * rhs, self.2 * rhs)
    }
}, Vec3, &Vec3);

ops_impl_for!(Div<f64> => {
    type Output = Vec3;
    fn div(self, rhs: f64) -> Self::Output {
        Vec3(self.0 / rhs, self.1 / rhs, self.2 / rhs)
    }
}, Vec3, &Vec3);

ops_impl_for!(Add => {
    type Output = Vec3;
    fn add(self, rhs: Self) -> Self::Output {
        Vec3(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)    
    }
}, Vec3, &Vec3);

ops_impl_for!(Sub => {
    type Output = Vec3;
    fn sub(self, rhs: Self) -> Self::Output {
        Vec3(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)    
    }
}, Vec3, &Vec3);
