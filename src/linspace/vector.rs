use num::{Float, Num};
use num::pow;
use std::fmt;
use std::ops::{Add, Sub, Mul, Neg};

/// Vector of 3 dimensions with a defined coordinates and origen
///
/// # Remarks
///
/// * This struct is implemented to be used with numerical types, not tested
///   for strings, bools, or other types.
/// * By default, the origen is [0, 0, 0] and currently this cannot be modified.
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
    #[inline]
    pub fn x(&self) -> N {
        self.x
    }

    /// Returns the `y` coordinate
    #[inline]
    pub fn y(&self) -> N {
        self.y
    }

    /// Returns the `z` coordinate
    #[inline]
    pub fn z(&self) -> N {
        self.z
    }

    /// Modifies the `X` coordinate
    ///
    /// # Arguments
    ///
    /// * `new_x`: new X value
    #[inline]
    pub fn set_x(&mut self, new_x: N) {
        self.x = new_x;
    }

    /// Modifies the `Y` coordinate
    ///
    /// # Arguments
    ///
    /// * `new_y`: new Y value
    #[inline]
    pub fn set_y(&mut self, new_y: N) {
        self.y = new_y;
    }

    // Modifies the `Z` coordinate
    ///
    /// # Arguments
    ///
    /// * `new_z`: new Z value
    #[inline]
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
    #[inline]
    pub fn set(&mut self, new_x: N, new_y: N, new_z: N) {
        self.x = new_x;
        self.y = new_y;
        self.z = new_z;
    }

    /// Initializes a Vector3D with default coordinates' values
    #[inline]
    pub fn new() -> Vector3D<N> where N: Default {
        Vector3D { x: N::default(),
                   y: N::default(),
                   z: N::default() }
    }

    /// Initializes a Vector3D with given coordinates
    ///
    /// # Arguments
    ///
    /// * `x`: X value
    /// * `y`: Y value
    /// * `z`: Z value
    #[inline]
    pub fn init(x: N, y: N, z: N) -> Vector3D<N> {
        Vector3D { x: x,
                   y: y,
                   z: z }
    }
    /// Scale a Vector with a given number
    ///
    /// # Arguments
    ///
    /// * `scalar`: scalar value
    #[inline]
    pub fn scale(&mut self, scalar: N) {
        self.x = self.x * scalar;
        self.y = self.y * scalar;
        self.z = self.z * scalar;
    }

    /// Scale a vector with a given vector
    ///
    /// # Arguments
    ///
    /// * `vec`: vector value
    #[inline]
    pub fn scale_vec(&mut self, vec: &Vector3D<N>) {
        self.x = self.x * vec.x();
        self.y = self.y * vec.y();
        self.z = self.z * vec.z();
    }

    /// Cross multiplication of two vectors
    ///
    /// # Arguments
    ///
    /// * `first`: first cross vector
    /// * `second`: second cross vector
    ///
    /// # Remarks
    ///
    /// * The `origin` of the output vector will be extracted from the `first` input vector
    #[inline]
    pub fn cross(first: &Vector3D<N>, second: &Vector3D<N>) -> Vector3D<N>
        where N: Default + Neg<Output = N> {

        let new_x: N = (first.y * second.z) - (first.z * second.y);
        let new_y: N = -((first.x * second.z) - (first.z * second.x));
        let new_z: N = (first.x * second.y) - (first.y * second.x);

        Vector3D { x: new_x, y: new_y, z: new_z }
    }

    /// Dot operation for two vectors
    ///
    /// # Arguments
    ///
    /// * `other`: second vector for the dot
    #[inline]
    pub fn dot(&self, other: &Vector3D<N>) -> N {
        ((self.x*other.x) + (self.y*other.y) + (self.z*other.z))
    }

    /// Normalize the vector
    ///
    /// # Remarks
    ///
    /// * Since this operation requires an `sqrt`, it's just available for float vectors
    #[inline]
    pub fn norm(&mut self) where N: Float {
        let value = (self.x*self.x) + (self.y*self.y) + (self.z*self.z);
        self.scale(value.sqrt().recip())
    }

    /// Return the Eucledian distance from vector
    #[inline]
    pub fn dist(&self) -> f64 where N: Into<f64> {
        (pow(self.x, 2) + pow(self.y, 2) + pow(self.z, 2)).into().sqrt()
    }
}

////////////////////////////////////////////////////////////////////////////////
// Traits
////////////////////////////////////////////////////////////////////////////////


/// Equivalence ´==´ implementation for Vector3D
impl<N: Copy + PartialEq> PartialEq for Vector3D<N> {
    fn eq(&self, other: &Vector3D<N>) -> bool {
        if (self.x == other.x) && (self.y == other.y) && (self.z == other.z) {
            return true;
        }
        false
    }
}

/// Add implementation `+` for Vector3D
///
/// # Remarks
///
/// * The `origin` of the output vector will be extracted from the `self` input vector
impl<N: Copy + Num> Add for Vector3D<N> {
    type Output = Vector3D<N>;

    fn add(self, other: Vector3D<N>) -> Vector3D<N> {
        Vector3D {x: self.x + other.x, y: self.y + other.y, z: self.z + other.z}
    }
}

/// Sub implementation `-` for Vector3D
///
/// # Remarks
///
/// * The `origin` of the output vector will be extracted from the `self` input vector
impl<N: Copy + Num> Sub for Vector3D<N> {
    type Output = Vector3D<N>;

    fn sub(self, other: Vector3D<N>) -> Vector3D<N> {
        Vector3D {x: self.x - other.x, y: self.y - other.y, z: self.z - other.z}
    }
}

/// Mul implementation `*` for Vector3D
///
/// # Remarks
///
/// * The `origin` of the output vector will be extracted from the `self` input vector
impl<N: Copy + Num> Mul for Vector3D<N> {
    type Output = Vector3D<N>;

    fn mul(self, other: Vector3D<N>) -> Vector3D<N> {
        Vector3D {x: self.x * other.x, y: self.y * other.y, z: self.z * other.z}
    }
}

impl<N: Copy + Num> Mul<N> for Vector3D<N> {
    type Output = Vector3D<N>;

    fn mul(self, other: N) -> Vector3D<N> {
        Vector3D {x: self.x * other, y: self.y * other, z: self.z * other}
    }
}


/// Display implementation for Vector3D
impl<N: Copy + Num> fmt::Display for Vector3D<N> where N: fmt::Display {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "( {x}, {y}, {z} )", x = self.x, y = self.y, z = self.z)
    }
}
