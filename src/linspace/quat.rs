use num::Num;
use num::pow;
use std::fmt;
use std::ops::{Add, Sub, Mul, Neg};

/// Quaternion represents a three dimensional component (x, y, z) with a definied
/// amount of rotation (w).
///
/// # Remarks
///
/// This struct is implemented to be used with numerical types.
#[derive(Clone, Copy)]
pub struct Quat<N: Copy> {
    x: N,
    y: N,
    z: N,
    w: N
}

////////////////////////////////////////////////////////////////////////////////
// Inherent methods
////////////////////////////////////////////////////////////////////////////////

impl<N: Copy + Num> Quat<N> { // implementation of Quat<N>

    /// Initializes a Matrix with default values
    #[inline]
    pub fn new() -> Quat<N> where N: Default {
        Quat { x: N::default(),
               y: N::default(),
               z: N::default(),
               w: N::default()}
    }

    /// Initializes a Quat with defined values
    ///
    /// # Arguments
    ///
    /// * `x`: X dimension value
    /// * `y`: Y dimension value
    /// * `z`: Z dimension value
    /// * `w`: rotation value
    #[inline]
    pub fn init(x: N, y: N, z: N, w: N) -> Quat<N> {
        Quat { x: x,
               y: y,
               z: z,
               w: w }
    }

    /// Returns X dimension value
    #[inline]
    pub fn x(&self) -> N {
        self.x
    }

    /// Returns Y dimension value
    #[inline]
    pub fn y(&self) -> N {
        self.y
    }

    /// Returns Z dimension value
    pub fn z(&self) -> N {
        self.z
    }

    /// Returns rotation value
    #[inline]
    pub fn w(&self) -> N {
        self.w
    }

    /// Modifies X dimension value
    ///
    /// # Arguments
    ///
    /// * `x`: new X dimension value
    #[inline]
    pub fn set_x(&mut self, x: N) {
        self.x = x;
    }

    /// Modifies Y dimension value
    ///
    /// # Arguments
    ///
    /// * `y`: new Y dimension value
    #[inline]
    pub fn set_y(&mut self, y: N) {
        self.y = y;
    }

    /// Modifies Z dimension value
    ///
    /// # Arguments
    ///
    /// * `z`: new Z dimension value
    #[inline]
    pub fn set_z(&mut self, z: N) {
        self.z = z;
    }

    /// Modifies rotation value
    ///
    /// # Arguments
    ///
    /// * `w`: new rotation value
    #[inline]
    pub fn set_w(&mut self, w: N) {
        self.w = w;
    }

    /// Modify all values at once
    ///
    /// # Arguments
    ///
    /// * `x`: new X dimension value
    /// * `y`: new Y dimension value
    /// * `z`: new Z dimension value
    /// * `w`: new rotation value
    #[inline]
    pub fn set(&mut self, x: N, y: N, z: N, w: N) {
        self.x = x;
        self.y = y;
        self.z = z;
        self.w = w;
    }

    /// Scale the Quat values
    ///
    /// # Arguments
    ///
    /// * `scalar`: value of the scalation
    #[inline]
    pub fn scale(&mut self, scalar: N) {
        self.x = self.x * scalar;
        self.y = self.y * scalar;
        self.z = self.z * scalar;
        self.w = self.w * scalar;
    }

    /// Returns the conjugation of a quaternion
    #[inline]
    pub fn conjugate(&self) -> Quat<N> where N: Neg<Output = N> {
        Quat {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: self.w
        }
    }

    /// Returns the magnitude of a quaternion
    #[inline]
    pub fn magnitude(&self) -> f64 where N: Into<f64> {
        (pow(self.x, 2) + pow(self.y, 2) + pow(self.z, 2) + pow(self.w, 2)).into().sqrt()
    }

    /// Returns the normalization of a quaternion in decimal values
    #[inline]
    pub fn norm(&self) -> Quat<f64> where N: Into<f64> {
        let m: f64 = self.magnitude();
        Quat::<f64> {
            x: self.x.into() / m,
            y: self.y.into() / m,
            z: self.z.into() / m,
            w: self.w.into() / m
        }
    }
    
    /// Returns a quaternion representing the rotation of a three dimensional component
    ///
    /// # Arguments
    ///
    /// * `x`: x dimensional value
    /// * `y`: y dimensional value
    /// * `z`: z dimensional value
    /// * `angle`: angle of the rotation 
    #[inline]
    pub fn rotation(x: N, y: N, z: N, angle: f64) -> Quat<f64> 
        where N: Into<f64> {

        Quat::<f64> {
            x: x.into() * (angle * 0.5).sin(),
            y: y.into() * (angle * 0.5).sin(),
            z: z.into() * (angle * 0.5).sin(),
            w: (angle * 0.5).cos()
        }
    }
}

////////////////////////////////////////////////////////////////////////////////
// Traits
////////////////////////////////////////////////////////////////////////////////

/// Add `+` implementation for Quat
impl<N: Copy + Num> Add for Quat<N> {
    type Output = Quat<N>;

    fn add(self, other: Quat<N>) -> Quat<N> {
        Quat { x: self.x + other.x, 
               y: self.y + other.y,
               z: self.z + other.z,
               w: self.w + other.w }
    }
}

/// Sub `-` implementation for Quat
impl<N: Copy + Num> Sub for Quat<N> {
    type Output = Quat<N>;

    fn sub(self, other: Quat<N>) -> Quat<N> {
        Quat { x: self.x - other.x, 
               y: self.y - other.y,
               z: self.z - other.z,
               w: self.w - other.w }
    }
}

/// Negative `-` implementation for Quat
impl<N: Copy + Num + Neg<Output = N>> Neg for Quat<N> {
    type Output = Quat<N>;

    fn neg(self) -> Quat<N> {
        Quat { x: -self.x,
               y: -self.y,
               z: -self.z,
               w: -self.w }
    }
}

/// Multiplication `*` implementation for Quat
impl<N: Copy + Num> Mul for Quat<N> {
    type Output = Quat<N>;

    fn mul(self, other: Quat<N>) -> Quat<N> {
        Quat { x: self.w*other.x + self.x*other.w + self.y*other.z - self.z*other.y,
               y: self.w*other.y - self.x*other.z + self.y*other.w + self.z*other.x,
               z: self.w*other.z + self.x*other.y - self.y*other.x + self.z*other.w,
               w: self.w*other.w - self.x*other.x - self.y*other.y - self.z*other.z }
    }
}

/// Display implementation for Quat
impl<N: Copy + Num> fmt::Display for Quat<N> where N: fmt::Display {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "( {x}, {y}, {z}, {w} )", x = self.x, y = self.y, z = self.z, w = self.w)
    }
}