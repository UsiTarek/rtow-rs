use std::ops::{Add, AddAssign, Index, IndexMut, Mul, Sub, SubAssign};

pub type Point3 = Vec3;

#[derive(Debug, Clone, Copy)]
pub struct Vec3 {
    e: [f32; 3],
}

impl Vec3 {
    pub fn new_random() -> Vec3 {
        Vec3::new(
            super::random_float(),
            super::random_float(),
            super::random_float(),
        )
    }

    pub fn new_random_mm(min: f32, max: f32) -> Vec3 {
        Vec3::new(
            super::random_float_mm(min, max),
            super::random_float_mm(min, max),
            super::random_float_mm(min, max),
        )
    }

    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Self { e: [x, y, z] }
    }

    pub fn x(&self) -> f32 {
        self.e[0]
    }

    pub fn y(&self) -> f32 {
        self.e[1]
    }

    pub fn z(&self) -> f32 {
        self.e[2]
    }

    pub fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }
    pub fn length_squared(&self) -> f32 {
        dot(&self, &self)
    }

    pub fn scale(&self, factor: f32) -> Vec3 {
        Vec3::new(self.x() * factor, self.y() * factor, self.z() * factor)
    }

    pub fn unit(&self) -> Vec3 {
        self.scale(1.0 / self.length())
    }

    pub fn near_zero(&self) -> bool {
        let small_number = 1e-8;
        self.x().abs() < small_number
            && self.y().abs() < small_number
            && self.z().abs() < small_number
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            e: [self.x() + rhs.x(), self.y() + rhs.y(), self.z() + rhs.z()],
        }
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            e: [self.x() - rhs.x(), self.y() - rhs.y(), self.z() - rhs.z()],
        }
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Vec3) {
        self.e[0] += rhs.x();
        self.e[1] += rhs.y();
        self.e[2] += rhs.z();
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Vec3) {
        self.e[0] -= rhs.x();
        self.e[1] -= rhs.y();
        self.e[2] -= rhs.z();
    }
}

impl Mul for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Self) -> Self::Output {
        Vec3::new(self.x() * rhs.x(), self.y() * rhs.y(), self.z() * rhs.z())
    }
}

impl Index<usize> for Vec3 {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        if index <= 2 {
            &self.e[index as usize]
        } else {
            panic!("Trying to access index > 2 in Vec3.")
        }
    }
}

impl IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        if index <= 2 {
            &mut self.e[index as usize]
        } else {
            panic!("Trying to access index > 2 in Vec3.")
        }
    }
}

pub fn dot(lhs: &Vec3, rhs: &Vec3) -> f32 {
    (lhs.x() * rhs.x()) + (lhs.y() * rhs.y()) + (lhs.z() * rhs.z())
}

pub fn cross(lhs: &Vec3, rhs: &Vec3) -> Vec3 {
    Vec3::new(
        lhs.y() * rhs.z() - lhs.z() * rhs.y(),
        lhs.z() * rhs.x() - lhs.x() * rhs.z(),
        lhs.x() * rhs.y() - lhs.y() * rhs.x(),
    )
}
