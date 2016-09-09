use num::Num;
use num::pow;
use std::fmt;
use std::fmt::Display;

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

    /// Returns the Euclidean Distance between two points
    ///
    /// # Arguments
    ///
    /// * `a`: first point
    /// * `b`: second point
    pub fn eucl_distance(a: &Point3D<N>, b: &Point3D<N>) -> f64 where N: Into<f64> {
        let val: f64 = (pow((a.x() - b.x()), 2) +
                        pow((a.y() - b.y()), 2) +
                        pow((a.z() - b.z()), 2)).into();
        val.sqrt()
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

/// Display implementation for Point3D
impl<N: Copy + Num> fmt::Display for Point3D<N> where N: Display {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[ {x}, {y}, {z} ]", x = self.x, y = self.y, z = self.z)
    }
}
