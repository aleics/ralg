use num::Num;
use std::fmt;
use std::fmt::Display;
use std::ops::{Add, Sub, Mul};

#[derive(Clone, Copy)]
pub struct Vector3D<N: Copy> {
    x: N,
    y: N,
    z: N,
}

impl<N: Copy + Num> Vector3D<N> {
    pub fn x(&self) -> N {
        self.x
    }

    pub fn y(&self) -> N {
        self.y
    }

    pub fn z(&self) -> N {
        self.z
    }

    pub fn init() -> Vector3D<N> where N: Default {
        Vector3D {x: N::default(), y: N::default(), z: N::default()}
    }

    pub fn init_with_values(x: N, y: N, z: N) -> Vector3D<N> {
        Vector3D {x: x, y: y, z: z}
    }

    pub fn set_x(&mut self, new_x: N) {
        self.x = new_x;
    }

    pub fn set_y(&mut self, new_y: N) {
        self.y = new_y;
    }

    pub fn set_z(&mut self, new_z: N) {
        self.z = new_z;
    }

    pub fn set(&mut self, new_x: N, new_y: N, new_z: N) {
        self.x = new_x;
        self.y = new_y;
        self.z = new_z;
    }

    pub fn scalar_mul(&self, scalar: N) -> Vector3D<N> {
        Vector3D {x: self.x * scalar, y: self.y * scalar, z: self.z * scalar}
    }
}

impl<N: Copy + Num> Add for Vector3D<N> {
    type Output = Vector3D<N>;

    fn add(self, other: Vector3D<N>) -> Vector3D<N> {
        Vector3D {x: self.x + other.x, y: self.y + other.y, z: self.z + other.z}
    }
}

impl<N: Copy + Num> Sub for Vector3D<N> {
    type Output = Vector3D<N>;

    fn sub(self, other: Vector3D<N>) -> Vector3D<N> {
        Vector3D {x: self.x - other.x, y: self.y - other.y, z: self.z - other.z}
    }
}

impl<N: Copy + Num> Mul for Vector3D<N> {
    type Output = Vector3D<N>;

    fn mul(self, other: Vector3D<N>) -> Vector3D<N> {
        Vector3D {x: self.x * other.x, y: self.y * other.y, z: self.z * other.z}
    }
}

impl<N: Copy + Num> fmt::Display for Vector3D<N> where N: Display {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[ {x}, {y}, {z} ]", x = self.x, y = self.y, z = self.z)
    }
}
