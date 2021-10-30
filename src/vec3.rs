use std::{fmt::{self, Display}, ops::{Add, AddAssign, Div, Index, IndexMut, Mul, Range, Sub, SubAssign}};

use rand::{thread_rng, Rng};

pub type Point3 = Vec3;
pub type Color = Vec3;

pub struct Iter<'a>(std::slice::Iter<'a, f32>);
pub struct IterMut<'a>(std::slice::IterMut<'a, f32>);

#[derive(Clone, Copy, Debug)]
pub struct Vec3 {
    elems: [f32; 3],
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3 { elems: [x, y, z] }
    }

    pub fn rand() -> Vec3 {
        Self::rand_range(0.0..1.0)
    }

    pub fn rand_range(range: Range<f32>) -> Vec3 {
        Self::new(
            thread_rng().gen_range(range.clone()),
            thread_rng().gen_range(range.clone()),
            thread_rng().gen_range(range.clone())
        )
    }

        pub fn x(&self) -> f32 {
        self[0]
    }

    pub fn y(&self) -> f32 {
        self[1]
    }

    pub fn z(&self) -> f32 {
        self[2]
    }

    pub fn r(&self) -> f32 {
        self.x()
    }

    pub fn g(&self) -> f32 {
        self.y()
    }

    pub fn b(&self) -> f32 {
        self.z()
    }

    pub fn iter(&self) -> Iter {
        Iter::new(self)
    }

    pub fn iter_mut(&mut self) -> IterMut {
        IterMut::new(self)
    }

    pub fn length_squared(&self) -> f32 {
        self.dot(&self)
    }

    pub fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }

    pub fn unit(&self) -> Self {
        let length = self.length();
        let x = self[0] / length;
        let y = self[1] / length;
        let z = self[2] / length;
        Self::new(x, y, z)
    }

    pub fn cross(&self, rhs: &Self) -> Self {
        let x = (self[1] * rhs[2]) - (self[2] * rhs[1]);
        let y = (self[0] * rhs[2]) - (self[2] * rhs[0]);
        let z = (self[1] * rhs[2]) - (self[2] * rhs[1]);
        Self::new(x, y, z)
    }

    pub fn dot(&self, rhs: &Self) -> f32 {
        self[0] * rhs[0] + self[1] * rhs[1] + self[2] * rhs[2]
    }
}

impl Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}, {}, {}]", self[0], self[1], self[2])
    }
}

impl Default for Vec3 {
    fn default() -> Vec3 {
        return Vec3 { elems: [0.0; 3] };
    }
}

impl<'a> Iterator for Iter<'a> {
    type Item = &'a f32;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

impl<'a> Iterator for IterMut<'a> {
    type Item = &'a mut f32;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

impl Index<usize> for Vec3 {
    type Output = f32;

    fn index(&self, idx: usize) -> &Self::Output {
        &self.elems[idx]
    }
}

impl IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
        &mut self.elems[idx]
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Self::Output {
        Vec3::new(self[0] + rhs[0], self[1] + rhs[1], self[2] + rhs[2])
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Self::Output {
        Vec3::new(self[0] - rhs[0], self[1] - rhs[1], self[2] - rhs[2])
    }
}

impl Mul<f32> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f32) -> Self::Output {
        Self::new(self[0] * rhs, self[1] * rhs, self[2] * rhs)
    }
}

impl Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Self::Output::new(self * rhs[0], self * rhs[1], self * rhs[2])
    }
}

impl Div<f32> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f32) -> Self::Output {
        Self::new(self[0] / rhs, self[1] / rhs, self[2] / rhs)
    }
}

impl Div<Vec3> for f32 {
    type Output = Vec3;

    fn div(self, rhs: Vec3) -> Self::Output {
        Self::Output::new(self / rhs[0], self / rhs[1], self / rhs[2])
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs
    }
}

impl From<[f32; 3]> for Vec3 {
    fn from(arr: [f32; 3]) -> Self {
        Self::new(arr[0], arr[1], arr[2])
    }
}

impl Into<[f32; 3]> for Vec3 {
    fn into(self) -> [f32; 3] {
        self.elems
    }
}

impl<'a> Iter<'a> {
    fn new(vec: &'a Vec3) -> Self {
        Self {
            0: vec.elems.iter(),
        }
    }
}

impl<'a> IterMut<'a> {
    fn new(vec: &'a mut Vec3) -> Self {
        Self {
            0: vec.elems.iter_mut(),
        }
    }
}
