use rand;
use rand::Rng;
use num::{Num, ToPrimitive};
use std::cmp::Ord;
use std::fmt;
use std::fmt::Display;
use rand::distributions::range::SampleRange;

pub struct Vector<N: Copy> {
    values: Vec<N>,
    size: usize
}

impl<N: Copy> Vector<N> {
    pub fn init() -> Vector<N> {
        Vector::<N> {
            values: Vec::new(),
            size: 0 }
    }

    pub fn init_with_values(val: &Vec<N>) -> Vector<N> {
        let mut v = Vector::<N>::init();
        v.values = val.clone();
        v.size = val.len();
        v
    }

    pub fn zeros(size: usize) -> Vector<N> where N: Num {
        let mut v = Vector::<N>::init();
        v.values = vec![N::zero(); size];
        v.size = size;
        v
    }

    pub fn ones(size: usize) -> Vector<N> where N: Num {
        let mut v = Vector::<N>::init();
        v.values = vec![N::one(); size];
        v.size = size;
        v
    }

    pub fn random(size: usize, range: &[N; 2]) -> Vector<N> where N: Num + PartialOrd + SampleRange {
        if range.len() != 2 {
                panic!("just permitted range of size 2 (actual={})", range.len());
        }
        let mut v = Vector::<N>::init();
        for _ in 0..size {
            v.push(rand::thread_rng().gen_range(range[0], range[1] + N::one()));
        }
        v.size = size;
        v
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn push(&mut self, val: N) {
        self.values.push(val);
    }

    pub fn remove(&mut self, index: usize) -> N {
        self.values.remove(index)
    }

    pub fn pop(&mut self) -> Option<N> {
        self.values.pop()
    }

    pub fn swap(&mut self, a: usize, b: usize) {
        self.values.swap(a, b);
    }

    pub fn append(&mut self, other: &Vector<N>) {
        self.values.append(&mut other.values.clone());
    }

    pub fn clear(&mut self) {
        self.values.clear();
    }

    pub fn sort_min(&mut self) where N: Num + Ord {
        self.values.sort_by(|a, b| a.cmp(b));
    }

    pub fn sort_max(&mut self) where N: Num + Ord {
        self.values.sort_by(|a, b| b.cmp(a));
    }

    pub fn el(&self, idx: usize) -> N {
        self.values[idx]
    }

    pub fn set_el(&mut self, idx: usize, val: N) {
        self.values[idx] = val;
    }

    pub fn max(&self) -> (N, usize) where N: Num + Ord {
        let mut i: usize = 0;

        for (j, &el) in self.values.iter().enumerate() {
            if el > self.values[i] {
                i = j;
            }
        }

        (self.values[i], i)
    }

    pub fn min(&self) -> (N, usize) where N: Num + Ord {
        let mut i: usize = 0;

        for (j, &el) in self.values.iter().enumerate() {
            if el < self.values[i] {
                i = j;
            }
        }

        (self.values[i], i)
    }

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
