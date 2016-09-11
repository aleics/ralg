
use rand;
use rand::Rng;
use num::pow;
use num::{Num, NumCast, ToPrimitive};
use std::fmt;
use std::fmt::Display;
use std::cmp::{PartialEq};
use std::ops::{Add, Sub, Mul};
use rand::distributions::range::SampleRange;

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
    /// * `nr`: rows capacity
    /// * `nc`: columns capacity
    pub fn init_with_capacity(nr: usize, nc: usize) -> Matrix<N> {
        let mut vec: Vec<Vec<N>> = Vec::with_capacity(nr);
        for i in 0..vec.len() {
            vec[i] = Vec::with_capacity(nc);
        }
        Matrix::<N> { values: vec,
                      nrows: nr,
                      ncols: nc }
    }

    /// Initializes a Matrix variable with a defined values
    ///
    /// # Arguments
    ///
    /// * `val`: the matrix will be initialized with these values
    pub fn init_with_values(val: &Vec<Vec<N>>) -> Matrix<N> {
        let mut m = Matrix::<N>::init();
        m.values = val.clone();
        m.update_sizes();
        m
    }

    /// Creates a Matrix variable with random values with a defined size
    ///
    /// # Arguments
    ///
    /// * `size_rows`: number of rows
    /// * `size_columns`: number of columns
    /// * `range`: range of the values
    pub fn random(size_rows: usize, size_columns: usize, range: &[N; 2])
        -> Matrix<N> where N: Num + PartialOrd + SampleRange {

        if range.len() != 2 {
                panic!("just permitted range of size 2 (actual={})", range.len());
        }

        let mut m = Matrix::<N>::init();
        for _ in 0..size_rows {
            let mut col: Vec<N> = Vec::new();
            for _ in 0..size_columns {
                col.push(rand::thread_rng().gen_range(range[0], range[1] + N::one()));
            }
            m.values.push(col);
        }
        m.update_sizes();
        m
    }

    /// Creates a Matrix of 0s
    ///
    /// # Arguments
    ///
    /// `size_rows`: row's size
    /// `size_columns`: column's size
    pub fn zeros(size_rows: usize, size_columns: usize)
        -> Matrix<N> where N: Num + Default {

        let mut m = Matrix::<N>::init();
        for _ in 0..size_rows {
            m.push_row(vec![N::zero(); size_columns]);
        }
        m
    }

    /// Creates a Matrix of 1s
    ///
    /// # Arguments
    ///
    /// `size_rows`: row's size
    /// `size_columns`: column's size
    pub fn ones(size_rows: usize, size_columns: usize)
        -> Matrix<N> where N: Num + Default {

        let mut m = Matrix::<N>::init();
        for _ in 0..size_rows {
            m.push_row(vec![N::one(); size_columns]);
        }
        m
    }

    /// Creates a Matrix variable with the identity matrix values
    ///
    /// # Arguments
    ///
    /// * `size`: size of columns and rows (the identity matrix is always square)
    pub fn create_identity(size: usize) -> Matrix<N>
        where N: Num + NumCast + ToPrimitive {

        let mut m: Matrix<N> = Matrix::<N>::init_with_capacity(size, size);
        for i in 0..size {
            let mut col: Vec<N> = Vec::new();
            for j in 0..size {
                if i == j {
                    col.push(NumCast::from(1usize).unwrap());
                } else {
                    col.push(NumCast::from(0usize).unwrap());
                }
            }
            m.values.push(col);
        }
        m
    }

    /// Copies a matrix value
    ///
    /// # Arguments
    ///
    /// * `m`: matrix to be copied
    pub fn copy(&mut self, m: Matrix<N>) {
        self.values = m.values;
        self.ncols = m.ncols;
        self.nrows = m.nrows;
    }

    /// Returns the number of currently rows
    pub fn nrows(&self) -> usize {
        self.nrows
    }

    /// Returns the number of currently columns
    pub fn ncols(&self) -> usize {
        self.ncols
    }

    /// Returns a defined row by an index
    ///
    /// # Arguments
    ///
    /// * `index`: index of the row that wants to be returned
    pub fn row(&self, index: usize) -> Option<&Vec<N>> {
        for (i, item) in self.values.iter().enumerate() {
            if i == index {
                return Some(item);
            }
        }
        None
    }

    /// Modifies a row with given index and value
    ///
    /// # Arguments
    ///
    /// * `index`: index of the row that must be modified
    /// * `new_row`: new row value
    pub fn set_row(&mut self, index: usize, new_row: &Vec<N>) {
        if index >= self.nrows() {
            panic!("index out of range")
        }
        if new_row.len() != self.ncols() {
            panic!("input vector dimension mismatch");
        }

        self.values[index].clone_from(new_row);
    }

    /// Returns a defined column by an index
    ///
    /// # Arguments
    ///
    /// * `index`: index of the column that wants to be returned
    pub fn col(&self, index: usize) -> Option<Vec<N>> {
        if self.ncols == 0 {
            return None;
        }

        let mut ret: Vec<N> = Vec::new();
        for (_, item) in self.values.iter().enumerate() {
            ret.push(item[index]);
        }
        Some(ret)
    }

    /// Modifies a column with given index and value
    ///
    /// # Arguments
    ///
    /// * `index`: index of the column that must be modified
    /// * `new_col`: new column value
    pub fn set_col(&mut self, index: usize, new_col: &Vec<N>) {
        if index >= self.ncols() {
            panic!("index out of range");
        }
        if new_col.len() != self.nrows() {
            panic!("input vector dimension mismatch");
        }

        for i in 0..self.nrows() {
            self.values[i][index] = new_col[i];
        }
    }

    /// Returns an element by coordinates
    ///
    /// # Arguments
    ///
    /// * `i_row`: row's index
    /// * `i_col`: column's index
    pub fn get_element(&self, i_row: usize, i_col: usize) -> N {
        self.values[i_row][i_col]
    }

    /// Modifies an element of the matrix
    ///
    /// # Arguments
    ///
    /// * `i_row`: row's index
    /// * `i_col`: column's index
    /// * `val`: new element value
    pub fn set_element(&mut self, i_row: usize, i_col: usize, val: &N) {
        self.values[i_row][i_col] = *val;
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
    pub fn contains(&self, element: &N) -> Option<usize> where N: Num {

        for (i, item) in self.row_iter().enumerate() {
            if item.contains(element) {
                return Some(i)
            }
        }
        None
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

        self.values.iter().position(|item| item == row)
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

        for (i, icol) in self.col_iter().enumerate() {
            if icol == *column {
                return Some(i);
            }
        }
        None
    }

    // internal use
    fn update_sizes(&mut self) { // helping function to update the size if matrix is modified
        self.nrows = self.values.len();

        if self.nrows > 0 {
            self.ncols = self.values[0].len();
        } else { // if number of columns is 0, the number of rows must be also 0
            self.ncols = 0;
        }
    }

    /// Appends a row to the end of the matrix
    ///
    /// # Arguments
    ///
    /// * `row`: row to push
    pub fn push_row(&mut self, row: Vec<N>) {
        self.values.push(row);
        self.update_sizes();
    }


    /// Appends a column to the end of the matrix
    ///
    /// # Arguments
    ///
    /// * `col`: column to push
    pub fn push_col(&mut self, col: Vec<N>) {
        if self.ncols > 0 && self.nrows > 0 {
            if self.nrows != col.len() {
                panic!("invalid size for a row. ncols = {}, row.len() = {}", self.ncols, col.len())
            }

            for (i, item) in col.iter().enumerate() {
                self.values[i].push(*item);
            }
        } else { // if self is empty -> push the first row
            for (i, item) in col.iter().enumerate() {
                self.values.push(Vec::new()); // initializye column vector
                self.values[i].push(*item); // push element
            }
        }
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

        let mut new_values: Vec<Vec<N>> = Vec::new();
        for (i, item) in self.values.iter().enumerate() {
            if i != index {
                new_values.push(item.clone());
            }
        }
        self.values = new_values;
        self.update_sizes();
    }

    /// Removes a column of the matrix by its index
    ///
    /// # Arguments
    ///
    /// `index`: index of the column that has to be removed
    pub fn pop_col(&mut self, index: usize) {
        if index >= self.ncols {
            panic!("invalid index({}). ncols = '{}'", index, self.nrows)
        }

        for item in self.values.iter_mut() {
            item.remove(index);
        }
        self.update_sizes();
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
        self.values.swap(index_a, index_b);
    }


    /// Swaps the position of two columns in the matrix
    ///
    /// # Arguments
    ///
    /// * `index_a`: index of the first row that has to be swapped
    /// * `index_b`: index of the second row that has to be swapped
    pub fn swap_col(&mut self, index_a: usize, index_b: usize) {
        if (index_a >= self.ncols) || (index_b >= self.ncols) {
            panic!("invalid indexes(index_a = {}, index_b = {}). ncols = {}",
                                    index_a, index_b, self.ncols);
        }
        for item in self.values.iter_mut() {
            item.swap(index_a, index_b);
        }
    }

    /// Clears a Matrix
    pub fn clear(&mut self) {
        self.values.clear();
        self.update_sizes();
    }

    /// Returns a matrix showing at each coordinate if a member was
    /// equal to a give value
    ///
    /// # Arguments
    ///
    /// * `value`: value to compare
    pub fn equal_to(&self, value: &N) -> Matrix<bool>
        where N: Num + PartialEq<N> {

        let mut m = Matrix::<bool>::init();
        for col in self.row_iter() {
            let mut row_out: Vec<bool> = Vec::new();

            for el in col.iter() {
                if el == value { row_out.push(true); }
                else { row_out.push(false); }
            }

            m.push_row(row_out);
        }
        m
    }

    /// Returns a matrix showing if two matrix were equal
    ///
    /// # Arguments
    ///
    /// * `comp_matrix`: matrix to compare
    pub fn equal_to_matrix(&self, comp_matrix: &Matrix<N>) -> Matrix<bool>
        where N: Num + PartialEq<N> {

        if (self.ncols != comp_matrix.ncols()) || (self.nrows != comp_matrix.nrows()) {
            panic!("sizes not equal!");
        }

        let mut m = Matrix::<bool>::init();
        for (i, row) in self.values.iter().enumerate() {
            let mut row_out: Vec<bool> = Vec::new();

            for (j, el) in row.iter().enumerate() {
                if *el == comp_matrix.values[i][j] {
                    row_out.push(true);
                } else {
                    row_out.push(false);
                }
            }

            m.push_row(row_out);
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
        where N: Num + PartialOrd<N> {

        let mut m = Matrix::<bool>::init();
        for row in self.row_iter() {
            let mut row_out: Vec<bool> = Vec::new();

            for el in row.iter() {
                if el > value {
                    row_out.push(true);
                } else {
                    row_out.push(false);
                }
            }

            m.push_row(row_out);
        }
        m
    }

    /// Returns a matrix comparing two matrixes and show which have bigger values
    /// in an specific coordinate
    ///
    /// # Arguments
    ///
    /// * `comp_matrix`: matrix to compare
    pub fn bigger_than_matrix(&self, comp_matrix: &Matrix<N>) -> Matrix<bool>
        where N: Num + PartialOrd<N> {

        if (self.ncols != comp_matrix.ncols()) || (self.nrows != comp_matrix.nrows()) {
            panic!("sizes not equal!");
        }

        let mut m = Matrix::<bool>::init();
        for (i, row) in self.row_iter().enumerate() {
            let mut row_out: Vec<bool> = Vec::new();

            for (j, el) in row.iter().enumerate() {
                if *el > comp_matrix.values[i][j] {
                    row_out.push(true);
                } else {
                    row_out.push(false);
                }
            }

            m.push_row(row_out);
        }
        m
    }

    /// Returns the product of an scalar multiplication with the current matrix
    ///
    /// # Arguments
    ///
    /// * `scalar`: number to multiply
    pub fn scalar_mul(&self, scalar: N) -> Matrix<N> where N: Num {
        let mut m: Matrix<N> = Matrix::<N>::init();

        for col in self.row_iter() {
            let mut new_row: Vec<N> = Vec::new();

            for el in col.iter() {
                new_row.push(scalar * (*el));
            }

            m.values.push(new_row);
        }
        m.update_sizes();
        m
    }

    /// Transposes a Matrix
    pub fn transpose(&mut self) {
        let mut res: Matrix<N> = Matrix::<N>::init();
        for item in self.values.iter() {
            res.push_col(item.clone());
        }

        res.update_sizes();
        self.copy(res);
    }

    /// Returns the diagonal of a Matrix
    pub fn get_diagonal(&self) -> Matrix<N> where N: Num + Default {

        if self.ncols != self.nrows {
            panic!("get_diagonal just available for square matrices");
        }

        let mut m: Matrix<N> = Matrix::<N>::init_with_capacity(self.ncols, self.nrows);
        for i in 0..m.nrows {
            let mut row: Vec<N> = Vec::new();
            for j in 0..m.ncols {
                if i != j {
                    row.push(N::default());
                } else {
                    row.push(self.values[i][j]);
                }
            }
            m.values.push(row);
        }
        m
    }

    /// Returns a submatrix of a matrix
    ///
    /// # Arguments
    ///
    /// * `range_row`: row's range of the submatrix
    /// * `range_col`: column's range of the submatrix
    pub fn submatrix(&self, range_row: &[usize; 2], range_col: &[usize; 2]) -> Matrix<N> {
        if range_col[0] > range_col[1] || range_row[0] > range_row[1] {
            panic!("please use ascendent ranges. For example '[0 3]'");
        }
        if range_col[1] > self.ncols || range_row[1] > self.nrows {
            panic!("index out of range");
        }

        let mut submatrix: Matrix<N> = Matrix::<N>::init_with_capacity(self.ncols, self.nrows);
        for i in range_row[0]..(range_row[1]+1) {
            let mut row: Vec<N> = Vec::new();
            for j in range_col[0]..(range_col[1]+1) {
                row.push(self.values[i][j]);
            }
            submatrix.values.push(row);
        }
        submatrix.update_sizes();
        submatrix
    }

    /// Returns a matrix with the Euclidean Distance between the rows
    ///
    /// # Remarks
    ///
    /// * The result matrix will be an square matrix with size of the columns
    ///   of the given matrix
    pub fn eucl_distance_row(&self) -> Matrix<f64> where N: Into<f64> + Num {
        let mut m: Matrix<f64> = Matrix::<f64>::init();

        for (_, row_i) in self.values.iter().enumerate() {
            let mut eucl_row: Vec<f64> = Vec::new();

            for (_, row_j) in self.values.iter().enumerate() {
                let mut val: f64 = 0.0;
                for x in 0..self.ncols {
                    val = val + (pow(row_i[x] - row_j[x], 2)).into();
                }
                eucl_row.push(val.sqrt());
            }
            m.push_col(eucl_row);
        }
        m.update_sizes();
        m
    }


    /// Returns a IteratorCol with a defined index
    ///
    /// # Arguments
    ///
    /// * `n`: index that the iterator will point to
    pub fn col_iter_at(&self, n: usize) -> IteratorCol<N> {
        IteratorCol {
            m: self,
            index: n
        }
    }

    /// Returns a IteratorCol pointing to the initial value
    pub fn col_iter(&self) -> IteratorCol<N> {
        self.col_iter_at(0)
    }

    /// Returns a IteratorRow with a defined index
    ///
    /// # Arguments
    ///
    /// * `n`: index that the iterator will point to
    pub fn row_iter_at(&self, n: usize) -> IteratorRow<N> {
        IteratorRow {
            m: self,
            index: n
        }
    }

    /// Returns a IteratorRow pointing to the initial value
    pub fn row_iter(&self) -> IteratorRow<N> {
        self.row_iter_at(0)
    }

    /// Returns a IteratorElement with a defined index
    ///
    /// # Arguments
    ///
    /// * `n`: index that the iterator will point to
    pub fn el_iter_at(&self, n: usize) -> IteratorElement<N> {
        IteratorElement {
            m: self,
            index: n
        }
    }

    /// Returns a IteratorElement pointing to the initial value
    pub fn el_iter(&self) -> IteratorElement<N> {
        self.el_iter_at(0)
    }
}

////////////////////////////////////////////////////////////////////////////////
// Traits
////////////////////////////////////////////////////////////////////////////////


/// Copy implementation for Matrix
impl<N: Copy> Copy for Matrix<N> where Vec<Vec<N>>: Copy + Clone { }

/// Clone implementation for Matrix
impl<N: Copy> Clone for Matrix<N> {
    fn clone(&self) -> Matrix<N> {
        let mut m: Matrix<N> = Matrix::<N>::init();
        m.values = self.values.clone();
        m.ncols = self.ncols;
        m.nrows = self.nrows;

        m
    }
}

/// Equivalence ´==´ implementation for Matrix
impl<N: Copy + PartialEq> PartialEq for Matrix<N> {
    fn eq(&self, other: &Matrix<N>) -> bool {
        for i in 0..self.nrows {
            for j in 0..self.ncols {
                if self.values[i][j] != other.values[i][j] {
                    return false;
                }
            }
        }
        true
    }
}

/// Addition ´+´ implementation for Matrix
impl<'a, N: Copy> Add for &'a Matrix<N> where N: Num + Add {
    type Output = Matrix<N>;

    fn add(self, other: &'a Matrix<N>) -> Matrix<N> {
        if (self.nrows != other.nrows) || (self.ncols != other.ncols) {
            panic!("sizes not compatible!");
        }

        let mut res: Matrix<N> = Matrix::<N>::init();
        for i in 0..self.nrows {

            let mut res_row: Vec<N> = Vec::new();
            for j in 0..self.ncols {
                let val = self.values[i][j] + other.values[i][j];
                res_row.push(val);
            }
            res.values.push(res_row);
        }
        res.ncols = self.ncols;
        res.nrows = self.nrows;

        res
    }
}

// Addition ´+´ implementation for Matrix
impl<N: Copy> Add for Matrix<N> where N: Num + Add {
    type Output = Matrix<N>;

    fn add(self, other: Matrix<N>) -> Matrix<N> {
        (&self) + (&other)
    }
}

/// Substraction ´-´ implementation for Matrix
impl<'a, N: Copy> Sub for &'a Matrix<N> where N: Num + Sub {
    type Output = Matrix<N>;

    fn sub(self, other: &'a Matrix<N>) -> Matrix<N> {
        if (self.nrows != other.nrows) || (self.ncols != other.ncols) {
            panic!("sizes not compatible!");
        }

        let mut res: Matrix<N> = Matrix::<N>::init();
        for i in 0..self.nrows {

            let mut res_row: Vec<N> = Vec::new();
            for j in 0..self.ncols {
                let val = self.values[i][j] - other.values[i][j];
                res_row.push(val);
            }
            res.values.push(res_row);
        }
        res.ncols = self.ncols;
        res.nrows = self.nrows;

        res
    }
}

/// Substraction ´-´ implementation for Matrix
impl<N: Copy> Sub for Matrix<N> where N: Num + Sub {
    type Output = Matrix<N>;

    fn sub(self, other: Matrix<N>) -> Matrix<N> {
        (&self) - (&other)
    }
}

/// Multiplication `*` implementation for &Matrix<N>
// Note: it will be good to implement Mul using parallelism
impl<'a, N: Copy + Default> Mul for &'a Matrix<N> where N: Num + Copy {
    type Output = Matrix<N>;

    fn mul(self, other: &'a Matrix<N>) -> Matrix<N> {
        if self.nrows != other.ncols {
            panic!("matrix dimension mismatch")
        } else {
            let mut res: Matrix<N> = Matrix::<N>::init();

            for irs in 0..self.nrows {
                let mut new_row: Vec<N> = Vec::new();
                for ico in 0..other.ncols {
                    let rs: Vec<N>;
                    let result_r = self.row(irs);
                    match result_r {
                        Some(x) => rs = x.clone(),
                        None => panic!("col of input matrix not found {}", irs),
                    }

                    let co: Vec<N>;
                    let result_c = other.col(ico);
                    match result_c {
                        Some(x) => co = x,
                        None => panic!("column of input matrix not found {}", ico),
                    }

                    if rs.len() != co.len() {
                        panic!("size of picked row/column not equal")
                    }

                    let mut sum: N = N::default();
                    for i in 0..rs.len() {
                        sum = sum + (rs[i] * co[i]);
                    }

                    new_row.push(sum);
                }
                res.push_row(new_row);
            }
            res
        }
    }
}

/// Multiplication `*` implementation for Matrix<N> (clone() must be used)
impl<N: Copy + Default> Mul for Matrix<N> where N: Num + Copy {
    type Output = Matrix<N>;

    fn mul(self, other: Matrix<N>) -> Matrix<N> {
        &self * &other
    }
}

/// Display implementation for Matrix
impl<N: Copy> fmt::Display for Matrix<N> where N: Display {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(writeln!(f, "{{"));
        for row in self.values.iter() {
            try!(write!(f, "[ "));
            for el in row.iter() {
                try!(write!(f, "{} ", el));
            }
            try!(writeln!(f, "]"));
        }
        try!(writeln!(f, "}}"));
        write!(f, "size: {row} x {col}", row = self.nrows, col = self.ncols)
    }
}

// --------------- Iterators ----------------------------------------

/// Definition of IteratorCol: the iterator for the columns dimension
pub struct IteratorCol<'a, N: 'a + Copy> {
    m: &'a Matrix<N>,
    index: usize
}

/// Implementation of the Iterator for IteratorCol
impl<'a, N: Clone + Copy> Iterator for IteratorCol<'a, N> {
    type Item = Vec<N>;

    fn next(&mut self) -> Option<Self::Item> {
        self.index += 1;
        match self.index > self.m.ncols() {
            true => None,
            false => self.m.col(self.index - 1)
        }
    }
}

/// Definition of IteratorRow: the iterator for the row dimension
pub struct IteratorRow<'a, N: 'a + Copy> {
    m: &'a Matrix<N>,
    index: usize
}

/// Implementation of the Iterator for IteratorRow
impl<'a, N: Clone + Copy> Iterator for IteratorRow<'a, N> {
    type Item = &'a Vec<N>;

    fn next(&mut self) -> Option<Self::Item> {
        self.index += 1;
        match self.index > self.m.nrows() {
            true => None,
            false => self.m.row(self.index - 1)
        }
    }
}

/// Definition of IteratorElement: the iterator for each element
pub struct IteratorElement<'a, N: 'a + Copy> {
    m: &'a Matrix<N>,
    index: usize
}

/// Implementation of the Iterator for IteratorElement
impl<'a, N: Clone + Copy> Iterator for IteratorElement<'a, N> {
    type Item = N;

    fn next(&mut self) -> Option<Self::Item> {
        let row_i: usize = self.index / self.m.ncols();
        let col_i: usize = self.index % self.m.ncols();

        self.index += 1;

        Some(self.m.get_element(row_i, col_i))
    }
}
