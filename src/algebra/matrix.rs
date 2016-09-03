extern crate num;

use std::cmp::{PartialEq};

pub struct Matrix<N: Copy> {
    values: Vec<Vec<N>>,
    nrows: usize,
    ncols: usize,
}

pub fn init<N: Copy>() -> Matrix<N> {
    Matrix::<N> { values: Vec::new(),
                  nrows: 0,
                  ncols: 0 }
}

pub fn init_with_capacity<N: Copy>(nc: usize, nr: usize) -> Matrix<N>{
    let mut vec: Vec<Vec<N>> = Vec::with_capacity(nc);
    for i in 0..vec.len() {
        vec[i] = Vec::with_capacity(nr);
    }
    Matrix::<N> { values: vec,
                  nrows: nr,
                  ncols: nc }
}

impl<N: Copy> Matrix<N> {
    pub fn nrows(&self) -> usize {
        self.nrows
    }

    pub fn ncols(&self) -> usize {
        self.ncols
    }

    pub fn get_col(&self, index: usize) -> Option<&Vec<N>> {
        for i in 0..self.values.len() {
            if i == index {
                return Some(&self.values[i]);
            }
        }
        None
    }

    pub fn get_row(self, index: usize) -> Option<Vec<N>> {
        let mut ret: Vec<N> = Vec::new();
        for i in 0..self.values.len() {
            ret.push(self.values[i][index]);
        }
        Some(ret)
    }

    pub fn get_element(&self, i_col: usize, i_row: usize) -> N {
        self.values[i_col][i_row]
    }

    pub fn push_col(&mut self, col: Vec<N>) {
        self.values.push(col);
        self.update_sizes();
    }

    pub fn push_row(&mut self, row: Vec<N>) {
        if self.ncols != row.len() {
            panic!("invalid size for a row ncols = {}, row.len() = {}", self.ncols, row.len())
        }
        for i in 0..row.len() {
            self.values[i].push(row[i]);
        }
        self.update_sizes();
    }

    fn update_sizes(&mut self) {
        self.ncols = self.values.len();
        self.nrows = self.values[0].len();
    }

    pub fn equal_to(&self, value: N) -> Matrix<bool>
        where N: num::Num + PartialEq<N> {

        let mut m = init::<bool>();
        for col in &self.values {
            let mut col_out: Vec<bool> = Vec::new();
            for el in col {
                if *el == value {
                    col_out.push(true);
                } else {
                    col_out.push(false);
                }
            }
            m.push_col(col_out);
        }
        m
    }

    pub fn equal_to_matrix(&self, input_matrix: &Matrix<N>) -> Matrix<bool>
        where N: num::Num + PartialEq<N> {

        if (self.ncols() != input_matrix.ncols()) || (self.nrows != input_matrix.nrows()) {
            panic!("sizes not equal!");
        }

        let mut m = init::<bool>();
        for i in 0..self.ncols() {
            let mut col_out: Vec<bool> = Vec::new();
            for j in 0..self.nrows() {
                if self.values[i][j] == input_matrix.values[i][j] {
                    col_out.push(true);
                } else {
                    col_out.push(false);
                }
            }
            m.push_col(col_out);
        }
        m
    }

    pub fn bigger_than(&self, value: N) -> Matrix<bool>
        where N: num::Num + PartialOrd<N> {

        let mut m = init::<bool>();
        for col in &self.values {
            let mut col_out: Vec<bool> = Vec::new();
            for el in col {
                if *el > value {
                    col_out.push(true);
                } else {
                    col_out.push(false);
                }
            }
            m.push_col(col_out);
        }
        m
    }

    pub fn bigger_than_matrix(&self, input_matrix: &Matrix<N>) -> Matrix<bool>
        where N: num::Num + PartialOrd<N> {

        if (self.ncols() != input_matrix.ncols()) || (self.nrows != input_matrix.nrows()) {
            panic!("sizes not equal!");
        }

        let mut m = init::<bool>();
        for i in 0..self.ncols() {
            let mut col_out: Vec<bool> = Vec::new();
            for j in 0..self.nrows() {
                if self.values[i][j] > input_matrix.values[i][j] {
                    col_out.push(true);
                } else {
                    col_out.push(false);
                }
            }
            m.push_col(col_out);
        }
        m
    }
}
