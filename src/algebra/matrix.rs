extern crate num;

use std::cmp::{PartialEq};

/// Matrix with a defined number of rows and columns that can
/// add, remove and edit values.
///
/// # Remarks
///
/// This struct is implemented to be used with numerical types, not tested
/// for strings, bools, or other types.
pub struct Matrix<N: Copy> {
    values: Vec<Vec<N>>,
    nrows: usize,
    ncols: usize,
}

////////////////////////////////////////////////////////////////////////////////
// Inherent methods
////////////////////////////////////////////////////////////////////////////////

impl<N: Copy> Matrix<N> { // implementation of Matrix<N>

    /// Initializes a Matrix variable with empty values
    pub fn init() -> Matrix<N> {
        Matrix::<N> { values: Vec::new(),
                      nrows: 0,
                      ncols: 0 }
    }

    /// Initializes a Matrix variable with a defined capacity for the rows
    /// and the columns
    ///
    /// # Arguments
    ///
    /// * `nc`: columns capacity
    /// * `nr`: rows capacity
    pub fn init_with_capacity(nc: usize, nr: usize) -> Matrix<N>{
        let mut vec: Vec<Vec<N>> = Vec::with_capacity(nc);
        for i in 0..vec.len() {
            vec[i] = Vec::with_capacity(nr);
        }
        Matrix::<N> { values: vec,
                      nrows: nr,
                      ncols: nc }
    }

    /// Returns the number of currently rows
    pub fn nrows(&self) -> usize {
        self.nrows
    }

    /// Returns the number of currently columns
    pub fn ncols(&self) -> usize {
        self.ncols
    }

    /// Returns a defined column by an index
    ///
    /// # Arguments
    ///
    /// * `index`: index of the column that wants to be returned
    pub fn get_col(&self, index: usize) -> Option<&Vec<N>> {
        for (i, item) in self.values.iter().enumerate() {
            if i == index {
                return Some(item);
            }
        }
        None
    }

    /// Returns a defined row by an index
    ///
    /// # Arguments
    ///
    /// * `index`: index of the row that wants to be returned
    pub fn get_row(&self, index: usize) -> Option<Vec<N>> {
        if self.ncols == 0 {
            return None;
        }

        let mut ret: Vec<N> = Vec::new();
        for (_, item) in self.values.iter().enumerate() {
            ret.push(item[index]);
        }
        Some(ret)
    }

    /// Returns an element by coordinates
    ///
    /// # Arguments
    ///
    /// * `i_col`: column's index
    /// * `i_row`: row's index
    pub fn get_element(&self, i_col: usize, i_row: usize) -> N {
        self.values[i_col][i_row]
    }

    /// Returns the index of an element if it's present on the matrix
    ///
    /// # Arguments
    ///
    /// * `element`: element's value
    ///
    /// # Remarks
    ///
    /// If the value is found then `Some(usize)` is returned, if not `None`
    pub fn contains(&self, element: &N) -> Option<usize> where N: num::Num {

        for (i, item) in self.values.iter().enumerate() {
            if item.contains(element) {
                return Some(i)
            }
        }
        None
    }

    /// Returns the index of a column if it's present on the matrix
    ///
    /// # Arguments
    ///
    /// * `column`: column's value
    ///
    /// # Remarks
    ///
    /// If the value is found then `Some(usize)` is returned, if not `None`
    pub fn contains_col(&self, column: &Vec<N>) -> Option<usize>
        where Vec<N>: PartialEq {

        self.values.iter().position(|item| item == column)
    }

    /// Returns the index of a row if it's present on the matrix
    ///
    /// # Arguments
    ///
    /// * `row`: row's value
    ///
    /// # Remarks
    ///
    /// If the value is found then `Some(usize)` is returned, if not `None`
    pub fn contains_row(&self, row: &Vec<N>) -> Option<usize>
        where Vec<N>: PartialEq {

        for i in 0..self.nrows {
            let res = self.get_row(i);
            let mut _val: Vec<N> = Vec::new();

            match res {
                Some(x) => _val = x.clone(),
                None => panic!("row of index '{}' not present on the matrix", i),
            }

            if &_val == row {
                return Some(i);
            }
        }
        None
    }

    fn update_sizes(&mut self) { // helping function to update the size if matrix is modified
        self.ncols = self.values.len();
        self.nrows = self.values[0].len();
    }

    /// Appends a column to the end of the matrix
    ///
    /// # Arguments
    ///
    /// * `col`: column to push
    pub fn push_col(&mut self, col: Vec<N>) {
        self.values.push(col);
        self.update_sizes();
    }

    /// Appends a row to the end of the matrix
    ///
    /// # Arguments
    ///
    /// * `row`: row to push
    pub fn push_row(&mut self, row: Vec<N>) {
        if self.ncols != row.len() {
            panic!("invalid size for a row. ncols = {}, row.len() = {}", self.ncols, row.len())
        }

        for (i, item) in row.iter().enumerate() {
            self.values[i].push(*item);
        }
        self.update_sizes();
    }

    /// Removes a column of the matrix by its index
    ///
    /// # Arguments
    ///
    /// `index`: index of the column that has to be removed
    pub fn pop_col(&mut self, index: usize) {
        if index >= self.ncols {
            panic!("invalid index({}). ncols = '{}'", index, self.ncols)
        }

        let mut new_values: Vec<Vec<N>> = Vec::new();
        for (i, item) in self.values.iter().enumerate() {
            if i != index {
                new_values.push(item.clone());
            }
        }
        self.values = new_values;
        self.update_sizes();
    }

    /// Removes a row of the matrix by its index
    ///
    /// # Arguments
    ///
    /// `index`: index of the row that has to be removed
    pub fn pop_row(&mut self, index: usize) {
        if index >= self.nrows {
            panic!("invalid index({}). nrows = '{}'", index, self.nrows)
        }

        for item in self.values.iter_mut() {
            item.remove(index);
        }
        self.update_sizes();
    }

    /// Swaps the position of two columns in the matrix
    ///
    /// # Arguments
    ///
    /// * `index_a`: index of the first column that has to be swapped
    /// * `index_b`: index of the second column that has to be swapped
    pub fn swap_col(&mut self, index_a: usize, index_b: usize) {
        if (index_a >= self.ncols) || (index_b >= self.ncols) {
            panic!("invalid indexes(index_a = {}, index_b = {}). ncols = {}",
                                    index_a, index_b, self.ncols);
        }
        self.values.swap(index_a, index_b);
    }

    /// Swaps the position of two rows in the matrix
    ///
    /// # Arguments
    ///
    /// * `index_a`: index of the first row that has to be swapped
    /// * `index_b`: index of the second row that has to be swapped
    pub fn swap_row(&mut self, index_a: usize, index_b: usize) {
        if (index_a >= self.nrows) || (index_b >= self.nrows) {
            panic!("invalid indexes(index_a = {}, index_b = {}). nrows = {}",
                                    index_a, index_b, self.nrows);
        }
        for item in self.values.iter_mut() {
            item.swap(index_a, index_b);
        }
    }

    /// Returns a matrix showing at each coordinate if a member was
    /// equal to a give value
    ///
    /// # Arguments
    ///
    /// * `value`: value to compare
    pub fn equal_to(&self, value: &N) -> Matrix<bool>
        where N: num::Num + PartialEq<N> {

        let mut m = Matrix::<bool>::init();
        for col in self.values.iter() {
            let mut col_out: Vec<bool> = Vec::new();

            for el in col.iter() {
                if el == value { col_out.push(true); }
                else { col_out.push(false); }
            }

            m.push_col(col_out);
        }
        m
    }

    /// Returns a matrix showing if two matrix were equal
    ///
    /// # Arguments
    ///
    /// * `comp_matrix`: matrix to compare
    pub fn equal_to_matrix(&self, comp_matrix: &Matrix<N>) -> Matrix<bool>
        where N: num::Num + PartialEq<N> {

        if (self.ncols != comp_matrix.ncols()) || (self.nrows != comp_matrix.nrows()) {
            panic!("sizes not equal!");
        }

        let mut m = Matrix::<bool>::init();
        for (i, col) in self.values.iter().enumerate() {
            let mut col_out: Vec<bool> = Vec::new();

            for (j, row) in col.iter().enumerate() {
                if *row == comp_matrix.values[i][j] {
                    col_out.push(true);
                } else {
                    col_out.push(false);
                }
            }

            m.push_col(col_out);
        }
        m
    }

    /// Returns a matrix showing at each coordinate if a member was
    /// bigger than the value given
    ///
    /// # Arguments
    ///
    /// * `value`: value to compare
    pub fn bigger_than(&self, value: &N) -> Matrix<bool>
        where N: num::Num + PartialOrd<N> {

        let mut m = Matrix::<bool>::init();
        for col in self.values.iter() {
            let mut col_out: Vec<bool> = Vec::new();

            for el in col.iter() {
                if el > value {
                    col_out.push(true);
                } else {
                    col_out.push(false);
                }
            }

            m.push_col(col_out);
        }
        m
    }

    // Returns a matrix comparing two matrixes and show which have bigger values
    // in an specific coordinate
    ///
    /// # Arguments
    ///
    /// * `comp_matrix`: matrix to compare
    pub fn bigger_than_matrix(&self, comp_matrix: &Matrix<N>) -> Matrix<bool>
        where N: num::Num + PartialOrd<N> {

        if (self.ncols != comp_matrix.ncols()) || (self.nrows != comp_matrix.nrows()) {
            panic!("sizes not equal!");
        }

        let mut m = Matrix::<bool>::init();
        for (i, col) in self.values.iter().enumerate() {
            let mut col_out: Vec<bool> = Vec::new();

            for (j, row) in col.iter().enumerate() {
                if *row > comp_matrix.values[i][j] {
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
