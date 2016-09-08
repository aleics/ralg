use num::Num;
use std::fmt;
use std::fmt::Display;
use std::ops::{Add, Sub, Mul};

/// Vector of 3 dimensions with a defined coordinates
///
/// # Remarks
///
/// This struct is implemented to be used with numerical types, not tested
/// for strings, bools, or other types.
#[derive(Clone, Copy)]
pub struct Vector3D<N: Copy> {
    x: N,
    y: N,
    z: N,
}

////////////////////////////////////////////////////////////////////////////////
// Inherent methods
////////////////////////////////////////////////////////////////////////////////

impl<N: Copy + Num> Vector3D<N> {

    /// Returns the `x` coordinate
    pub fn x(&self) -> N {
        self.x
    }

    /// Returns the `y` coordinate
    pub fn y(&self) -> N {
        self.y
    }

    /// Returns the `z` coordinate
    pub fn z(&self) -> N {
        self.z
    }

    /// Initializes a Vector3D with default coordinates' values
    pub fn init() -> Vector3D<N> where N: Default {
        Vector3D {x: N::default(), y: N::default(), z: N::default()}
    }

    /// Initializes a Vector3D with specified coordinates' values
    ///
    /// # Arguments
    ///
    /// * `x`: X coordinate
    /// * `y`: Y coordinate
    /// * `z`: Z coordinate
    pub fn init_with_values(x: N, y: N, z: N) -> Vector3D<N> {
        Vector3D {x: x, y: y, z: z}
    }

    /// Modifies the `X` coordinate
    ///
    /// # Arguments
    ///
    /// * `new_x`: new X value
    pub fn set_x(&mut self, new_x: N) {
        self.x = new_x;
    }

    /// Modifies the `Y` coordinate
    ///
    /// # Arguments
    ///
    /// * `new_y`: new Y value
    pub fn set_y(&mut self, new_y: N) {
        self.y = new_y;
    }

    // Modifies the `Z` coordinate
    ///
    /// # Arguments
    ///
    /// * `new_z`: new Z value
    pub fn set_z(&mut self, new_z: N) {
        self.z = new_z;
    }

    /// Modifies all coordinates
    ///
    /// # Arguments
    ///
    /// * `new_x`: new X value
    /// * `new_y`: new Y value
    /// * `new_z`: new Z value
    pub fn set(&mut self, new_x: N, new_y: N, new_z: N) {
        self.x = new_x;
        self.y = new_y;
        self.z = new_z;
    }

    /// Scalar multiplication of a vector
    ///
    /// # Arguments
    ///
    /// * `scalar`: scalar value
    pub fn scalar_mul(&self, scalar: N) -> Vector3D<N> {
        Vector3D {x: self.x * scalar, y: self.y * scalar, z: self.z * scalar}
    }
}

////////////////////////////////////////////////////////////////////////////////
// Traits
////////////////////////////////////////////////////////////////////////////////

/// Add implementation `+` for Vector3D
impl<N: Copy + Num> Add for Vector3D<N> {
    type Output = Vector3D<N>;

    fn add(self, other: Vector3D<N>) -> Vector3D<N> {
        Vector3D {x: self.x + other.x, y: self.y + other.y, z: self.z + other.z}
    }
}

/// Sub implementation `-` for Vector3D
impl<N: Copy + Num> Sub for Vector3D<N> {
    type Output = Vector3D<N>;

    fn sub(self, other: Vector3D<N>) -> Vector3D<N> {
        Vector3D {x: self.x - other.x, y: self.y - other.y, z: self.z - other.z}
    }
}

/// Mul implementation `*` for Vector3D
impl<N: Copy + Num> Mul for Vector3D<N> {
    type Output = Vector3D<N>;

    fn mul(self, other: Vector3D<N>) -> Vector3D<N> {
        Vector3D {x: self.x * other.x, y: self.y * other.y, z: self.z * other.z}
    }
}

/// Display implementation for Vector3D
impl<N: Copy + Num> fmt::Display for Vector3D<N> where N: Display {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[ {x}, {y}, {z} ]", x = self.x, y = self.y, z = self.z)
    }
}
