use rand;
use rand::Rng;
use num::{Num, ToPrimitive};
use std::cmp::Ord;
use std::fmt;
use std::fmt::Display;
use rand::distributions::range::SampleRange;

/// Vector with a defined number of elements that can
/// add, remove and edit values.
///
/// # Remarks
///
/// This struct is implemented to be used with numerical types, not tested
/// for strings, bools, or other types.
pub struct Vector<N: Copy> {
    values: Vec<N>
}

impl<N: Copy> Vector<N> {

    /// Initializes a Vector variable
    pub fn new() -> Vector<N> {
        Vector::<N> {
            values: Vec::new()}
    }

    /// Initializes a Vector variable with a defined values
    ///
    /// # Parameters
    ///
    /// * `val`: defined vector values
    pub fn init(val: &Vec<N>) -> Vector<N> {
        let mut v = Vector::<N>::new();
        v.values = val.clone();
        v
    }

    /// Generates a vector with zeros
    ///
    /// # Parameters
    ///
    /// * `size`: vector's size
    pub fn zeros(size: usize) -> Vector<N> where N: Num {
        let mut v = Vector::<N>::new();
        v.values = vec![N::zero(); size];
        v
    }

    /// Generates a vector with ones
    ///
    /// # Parameters
    ///
    /// * `size`: vector's size
    pub fn ones(size: usize) -> Vector<N> where N: Num {
        let mut v = Vector::<N>::new();
        v.values = vec![N::one(); size];
        v
    }

    ///  Generates a random vector within a range
    ///
    /// # Parameters
    ///
    /// * `size`: vector's size
    /// * `range`: values range
    pub fn random(size: usize, range: &[N; 2]) -> Vector<N> where N: Num + PartialOrd + SampleRange {
        if range.len() != 2 {
                panic!("just permitted range of size 2 (actual={})", range.len());
        }
        let mut v = Vector::<N>::new();
        for _ in 0..size {
            v.push(rand::thread_rng().gen_range(range[0], range[1] + N::one()));
        }
        v
    }

    /// Returns the size of Vector
    pub fn size(&self) -> usize {
        self.values.len()
    }

    /// Adds a new element to the end of Vector
    ///
    /// # Parameters
    ///
    /// * `val`: new element's value
    pub fn push(&mut self, val: N) {
        self.values.push(val);
    }

    /// Removes an element in a defined index
    ///
    /// # Parameters
    ///
    /// * `index`: deleting index
    pub fn remove(&mut self, index: usize) {
        if self.size() > 0 {
            self.values.remove(index);
        }
    }

    /// Removes the last element of Vector
    pub fn pop(&mut self) -> Option<N> {
        if self.size() > 0 {
            return self.values.pop();
        }
        None
    }

    /// Swap two elements with each other
    ///
    /// # Parameters
    ///
    /// * `a`: first element's index
    /// * `b`: second element's index
    pub fn swap(&mut self, a: usize, b: usize) {
        self.values.swap(a, b);
    }

    /// Adds a vector's values to a Vector
    ///
    /// # Parameters
    ///
    /// * `other`: vector's values that will be added
    ///
    /// # Remarks
    ///
    /// * The introduced vector won't lost the values
    pub fn append(&mut self, other: &Vector<N>) {
        self.values.append(&mut other.values.clone());
    }

    /// Clears a vector
    pub fn clear(&mut self) {
        self.values.clear();
    }

    /// Orders a vector from the minimum to maximum value
    pub fn sort_min(&mut self) where N: Num + Ord {
        self.values.sort_by(|a, b| a.cmp(b));
    }

    /// Orders a vector from the maximum to minimum value
    pub fn sort_max(&mut self) where N: Num + Ord {
        self.values.sort_by(|a, b| b.cmp(a));
    }

    /// Returns an element value
    ///
    /// # Parameters
    ///
    /// * `idx`: element's index to return
    pub fn el(&self, idx: usize) -> N {
        self.values[idx]
    }

    /// Modifies an element in a Vector
    ///
    /// # Parameters
    ///
    /// * `idx`: index of element to set
    ///* `val`: new value for the selected element
    pub fn set_el(&mut self, idx: usize, val: N) {
        self.values[idx] = val;
    }

    /// Returns the maximal element and its index of a Vector
    pub fn max(&self) -> (N, usize) where N: Num + Ord {
        let mut i: usize = 0;

        for (j, &el) in self.values.iter().enumerate() {
            if el > self.values[i] {
                i = j;
            }
        }

        (self.values[i], i)
    }

    /// Returns the minimal element and its index of a Vector
    pub fn min(&self) -> (N, usize) where N: Num + Ord {
        let mut i: usize = 0;

        for (j, &el) in self.values.iter().enumerate() {
            if el < self.values[i] {
                i = j;
            }
        }

        (self.values[i], i)
    }

    /// Returns the median value of the Vector's elements value
    pub fn median(&self) -> f64 where N: Num + Default + ToPrimitive {
        let sum = self.values.iter().fold(N::zero(), |sum, &el| sum + el).to_f64().unwrap();
        (sum / self.values.len() as f64)
    }
}

/// Display implementation for Vector
impl<N: Copy> Display for Vector<N> where N: Display {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(write!(f, "[ "));
        for el in self.values.iter() {
            try!(write!(f, "{el} ",  el = el));
        }
        writeln!(f, "]")
    }
}
