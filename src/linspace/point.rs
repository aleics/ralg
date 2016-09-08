use num::Num;
use std::fmt;
use std::fmt::Display;
use std::ops::{Add, Sub, Mul};

/// Point of 3 dimensions with a defined coordinates
///
/// # Remarks
///
/// This struct is implemented to be used with numerical types, not tested
/// for strings, bools, or other types.
#[derive(Clone, Copy)]
pub struct Point3D<N: Copy> {
    x: N,
    y: N,
    z: N,
}

////////////////////////////////////////////////////////////////////////////////
// Inherent methods
////////////////////////////////////////////////////////////////////////////////

impl<N: Copy + Num> Point3D<N> {

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

    /// Initializes a Point3D with default coordinates' values
    pub fn init() -> Point3D<N> where N: Default {
        Point3D {x: N::default(), y: N::default(), z: N::default()}
    }

    /// Initializes a Point3D with specified coordinates' values
    ///
    /// # Arguments
    ///
    /// * `x`: X coordinate
    /// * `y`: Y coordinate
    /// * `z`: Z coordinate
    pub fn init_with_values(x: N, y: N, z: N) -> Point3D<N> {
        Point3D {x: x, y: y, z: z}
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

    /// Scale a Point with a given number
    ///
    /// # Arguments
    ///
    /// * `scalar`: scalar value
    pub fn scale(&mut self, scalar: N) {
        self.x = self.x * scalar;
        self.y = self.y * scalar;
        self.z = self.z * scalar;
    }
}

////////////////////////////////////////////////////////////////////////////////
// Traits
////////////////////////////////////////////////////////////////////////////////

impl<N: Copy + PartialEq> PartialEq for Point3D<N> {
    fn eq(&self, other: &Point3D<N>) -> bool {
        if (self.x == other.x) && (self.y == other.y) && (self.z == other.z) {
            return true;
        }
        false
    }
}

/// Add implementation `+` for Point3D
impl<N: Copy + Num> Add for Point3D<N> {
    type Output = Point3D<N>;

    fn add(self, other: Point3D<N>) -> Point3D<N> {
        Point3D {x: self.x + other.x, y: self.y + other.y, z: self.z + other.z}
    }
}

/// Sub implementation `-` for Point3D
impl<N: Copy + Num> Sub for Point3D<N> {
    type Output = Point3D<N>;

    fn sub(self, other: Point3D<N>) -> Point3D<N> {
        Point3D {x: self.x - other.x, y: self.y - other.y, z: self.z - other.z}
    }
}

/// Mul implementation `*` for Point3D
impl<N: Copy + Num> Mul for Point3D<N> {
    type Output = Point3D<N>;

    fn mul(self, other: Point3D<N>) -> Point3D<N> {
        Point3D {x: self.x * other.x, y: self.y * other.y, z: self.z * other.z}
    }
}

/// Display implementation for Point3D
impl<N: Copy + Num> fmt::Display for Point3D<N> where N: Display {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[ {x}, {y}, {z} ]", x = self.x, y = self.y, z = self.z)
    }
}
