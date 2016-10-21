use num::Num;
use std::ops::{Add, Sub, Neg};

#[derive(Clone, Copy)]
pub struct Quat<N: Copy> {
    x: N,
    y: N,
    z: N,
    w: N
}

impl<N: Copy + Num> Quat<N> {
    pub fn init() -> Quat<N> where N: Default {
        Quat { x: N::default(),
               y: N::default(),
               z: N::default(),
               w: N::default()}
    }

    pub fn init_with_values(x: N, y: N, z: N, w: N) -> Quat<N> {
        Quat { x: x,
               y: y,
               z: z,
               w: w }
    }

    pub fn x(&self) -> N {
        self.x
    }

    pub fn y(&self) -> N {
        self.y
    }

    pub fn z(&self) -> N {
        self.z
    }

    pub fn w(&self) -> N {
        self.w
    }

    pub fn set_x(&mut self, x: N) {
        self.x = x;
    }

    pub fn set_y(&mut self, y: N) {
        self.y = y;
    }

    pub fn set_z(&mut self, z: N) {
        self.z = z;
    }

    pub fn set_w(&mut self, w: N) {
        self.w = w;
    }

    pub fn set(&mut self, x: N, y: N, z: N, w: N) {
        self.x = x;
        self.y = y;
        self.z = z;
        self.w = w;
    }

    pub fn scale(&mut self, scalar: N) {
        self.x = self.x * scalar;
        self.y = self.y * scalar;
        self.z = self.z * scalar;
        self.w = self.w * scalar;
    }
}

impl<N: Copy + Num> Add for Quat<N> {
    type Output = Quat<N>;

    fn add(self, other: Quat<N>) -> Quat<N> {
        Quat { x: self.x + other.x, 
               y: self.y + other.y,
               z: self.z + other.z,
               w: self.w + other.w }
    }
}

impl<N: Copy + Num> Sub for Quat<N> {
    type Output = Quat<N>;

    fn sub(self, other: Quat<N>) -> Quat<N> {
        Quat { x: self.x - other.x, 
               y: self.y - other.y,
               z: self.z - other.z,
               w: self.w - other.w }
    }
}

impl<N: Copy + Num + Neg<Output = N>> Neg for Quat<N> {
    type Output = Quat<N>;

    fn neg(self) -> Quat<N> {
        Quat {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: -self.w
        }
    }
}