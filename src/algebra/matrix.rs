extern crate num;

use std::cmp::{PartialEq};

// Matrix with a defined number of rows and columns that can be
// edited adding, removing and editing values.
pub struct Matrix<N: Copy> {
    values: Vec<Vec<N>>,
    nrows: usize,
    ncols: usize,
}

// Initializes a Matrix variable with empty values
pub fn init<N: Copy>() -> Matrix<N> {
    Matrix::<N> { values: Vec::new(),
                  nrows: 0,
                  ncols: 0 }
}

// Initializes a Matrix variable with a defined capacity for the rows
// and the columns
pub fn init_with_capacity<N: Copy>(nc: usize, nr: usize) -> Matrix<N>{
    let mut vec: Vec<Vec<N>> = Vec::with_capacity(nc);
    for i in 0..vec.len() {
        vec[i] = Vec::with_capacity(nr);
    }
    Matrix::<N> { values: vec,
                  nrows: nr,
                  ncols: nc }
}

impl<N: Copy> Matrix<N> { // implementation of Matrix<N>

    // Returns the number of currently rows
    pub fn nrows(&self) -> usize {
        self.nrows
    }

    // Return the number of currently columns
    pub fn ncols(&self) -> usize {
        self.ncols
    }

    // Return a defined column by an index
    pub fn get_col(&self, index: usize) -> Option<&Vec<N>> {
        for i in 0..self.ncols {
            if i == index {
                return Some(&self.values[i]);
            }
        }
        None
    }

    // Return a defined row by an index
    pub fn get_row(&self, index: usize) -> Option<Vec<N>> {
        if self.ncols == 0 {
            return None;
        }

        let mut ret: Vec<N> = Vec::new();
        for i in 0..self.ncols {
            ret.push(self.values[i][index]);
        }
        Some(ret)
    }

    // Return an element by coordinates
    pub fn get_element(&self, i_col: usize, i_row: usize) -> N {
        self.values[i_col][i_row]
    }

    fn update_sizes(&mut self) { // helping function to update the size if matrix is modified
        self.ncols = self.values.len();
        self.nrows = self.values[0].len();
    }

    // Push a column to the matrix
    pub fn push_col(&mut self, col: Vec<N>) {
        self.values.push(col);
        self.update_sizes();
    }

    // Push a row to the matrix
    pub fn push_row(&mut self, row: Vec<N>) {
        if self.ncols != row.len() {
            panic!("invalid size for a row. ncols = {}, row.len() = {}", self.ncols, row.len())
        }
        for i in 0..row.len() {
            self.values[i].push(row[i]);
        }
        self.update_sizes();
    }

    // Pop a column of the matrix
    pub fn pop_col(&mut self, index: usize) {
        if index >= self.ncols {
            panic!("invalid index({}). ncols = '{}'", index, self.ncols)
        }

        let mut new_values: Vec<Vec<N>> = Vec::new();
        for i in 0..self.ncols {
            if i != index {
                new_values.push(self.values[i].clone());
            }
        }
        self.values = new_values;
        self.update_sizes();
    }

    // Pop a row of the matrix
    pub fn pop_row(&mut self, index: usize) {
        if index >= self.nrows {
            panic!("invalid index({}). nrows = '{}'", index, self.nrows)
        }

        for i in 0..self.ncols {
            self.values[i].remove(index);
        }
        self.update_sizes();
    }

    // Swap the column of index_a with the column of index_b
    pub fn swap_col(&mut self, index_a: usize, index_b: usize) {
        if (index_a >= self.ncols) || (index_b >= self.ncols) {
            panic!("invalid indexes(index_a = {}, index_b = {}). ncols = {}",
                                    index_a, index_b, self.ncols);
        }
        self.values.swap(index_a, index_b);
    }

    // Swap the row of index_a with the row of index_b
    pub fn swap_row(&mut self, index_a: usize, index_b: usize) {
        if (index_a >= self.nrows) || (index_b >= self.nrows) {
            panic!("invalid indexes(index_a = {}, index_b = {}). nrows = {}",
                                    index_a, index_b, self.nrows);
        }
        for i in 0..self.ncols {
            self.values[i].swap(index_a, index_b);
        }
    }

    // Return a matrix showing at each coordinate if a member was
    // equal to the value given
    pub fn equal_to(&self, value: N) -> Matrix<bool>
        where N: num::Num + PartialEq<N> {

        let mut m = init::<bool>();
        for col in &self.values {

            let mut col_out: Vec<bool> = Vec::new();
            for el in col {

                if *el == value { col_out.push(true); }
                else { col_out.push(false); }
            }
            m.push_col(col_out);
        }
        m
    }

    // Return a matrix showing if two matrix were equal
    pub fn equal_to_matrix(&self, input_matrix: &Matrix<N>) -> Matrix<bool>
        where N: num::Num + PartialEq<N> {

        if (self.ncols != input_matrix.ncols()) || (self.nrows != input_matrix.nrows()) {
            panic!("sizes not equal!");
        }

        let mut m = init::<bool>();
        for i in 0..self.ncols {
            let mut col_out: Vec<bool> = Vec::new();
            for j in 0..self.nrows {
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

    // Return a matrix showing at each coordinate if a member was
    // bigger than the value given
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

    // Return a matrix comparing two matrixes and show which have bigger values
    // in an specific coordinate
    pub fn bigger_than_matrix(&self, input_matrix: &Matrix<N>) -> Matrix<bool>
        where N: num::Num + PartialOrd<N> {

        if (self.ncols != input_matrix.ncols()) || (self.nrows != input_matrix.nrows()) {
            panic!("sizes not equal!");
        }

        let mut m = init::<bool>();
        for i in 0..self.ncols {
            let mut col_out: Vec<bool> = Vec::new();
            for j in 0..self.nrows {
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
