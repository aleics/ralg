
use rand;
use rand::Rng;
use num::{Num, NumCast, ToPrimitive};
use std::fmt;
use std::fmt::Display;
use std::cmp::{PartialEq};
use std::default::Default;
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
    /// * `nc`: columns capacity
    /// * `nr`: rowsextern crate rand; capacity
    pub fn init_with_capacity(nc: usize, nr: usize) -> Matrix<N> {
        let mut vec: Vec<Vec<N>> = Vec::with_capacity(nc);
        for i in 0..vec.len() {
            vec[i] = Vec::with_capacity(nr);
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
    pub fn init_with_value(val: &Vec<Vec<N>>) -> Matrix<N> {
        let mut m = Matrix::<N>::init();
        m.values = val.clone();
        m.update_sizes();
        m
    }

    /// Creates a Matrix variable with random values with a defined size
    ///
    /// # Arguments
    ///
    /// * `size_columns`: number of columns
    /// * `size_rows`: number of rows
    /// * `range`: range of the values
    pub fn create_random(size_columns: usize, size_rows: usize, range: &[N; 2])
        -> Matrix<N> where N: Num + PartialOrd + SampleRange {

        if range.len() != 2 {
                panic!("just permitted range of size 2 (actual={})", range.len());
        }

        let mut m = Matrix::<N>::init();
        for _ in 0..size_columns {
            let mut col: Vec<N> = Vec::new();
            for _ in 0..size_rows {
                col.push(rand::thread_rng().gen_range(range[0], range[1]));
            }
            m.values.push(col);
        }

        m.update_sizes();
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
    pub fn contains(&self, element: &N) -> Option<usize> where N: Num {

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

    // internal use
    fn update_sizes(&mut self) { // helping function to update the size if matrix is modified
        self.ncols = self.values.len();

        if self.ncols > 0 {
            self.nrows = self.values[0].len();
        } else { // if number of columns is 0, the number of rows must be also 0
            self.nrows = 0;
        }
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
        if self.ncols > 0 && self.nrows > 0 {
            if self.ncols != row.len() {
                panic!("invalid size for a row. ncols = {}, row.len() = {}", self.ncols, row.len())
            }

            for (i, item) in row.iter().enumerate() {
                self.values[i].push(*item);
            }
        } else { // if self is empty -> push the first row
            for (i, item) in row.iter().enumerate() {
                self.values.push(Vec::new()); // initializye column vector
                self.values[i].push(*item); // push element
            }
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
        where N: Num + PartialEq<N> {

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
        where N: Num + PartialEq<N> {

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
        where N: Num + PartialOrd<N> {

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

    /// Returns the product of an scalar multiplication with the current matrix
    ///
    /// # Arguments
    ///
    /// * `scalar`: number to multiply
    pub fn scalar_mul(&self, scalar: N) -> Matrix<N> where N: Num {
        let mut m: Matrix<N> = Matrix::<N>::init();

        for col in self.values.iter() {
            let mut new_col: Vec<N> = Vec::new();

            for row in col.iter() {
                new_col.push(scalar * (*row));
            }

            m.values.push(new_col);
        }
        m.update_sizes();
        m
    }

    /// Transposes a Matrix
    pub fn transpose(&mut self) {
        let mut res: Matrix<N> = Matrix::<N>::init();
        for item in self.values.iter() {
            res.push_row(item.clone());
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
        for i in 0..m.ncols {
            let mut col: Vec<N> = Vec::new();
            for j in 0..m.nrows {
                if i != j {
                    col.push(N::default());
                } else {
                    col.push(self.values[i][j]);
                }
            }
            m.values.push(col);
        }
        m
    }

    /// Returns a submatrix of a matrix
    ///
    /// # Arguments
    ///
    /// * `range_col`: column's range of the submatrix
    /// * `range_row`: row's range of the submatrix
    pub fn submatrix(&self, range_col: &[usize; 2], range_row: &[usize; 2]) -> Matrix<N> {
        if range_col[0] > range_col[1] || range_row[0] > range_row[1] {
            panic!("please use ascendent ranges. For example '[0 3]'");
        }
        if range_col[1] > self.ncols || range_row[1] > self.nrows {
            panic!("index out of range");
        }

        let mut submatrix: Matrix<N> = Matrix::<N>::init_with_capacity(self.ncols, self.nrows);
        for i in range_col[0]..(range_col[1]+1) {
            let mut col: Vec<N> = Vec::new();
            for j in range_row[0]..(range_row[1]+1) {
                col.push(self.values[i][j]);
            }
            submatrix.values.push(col);
        }
        submatrix.update_sizes();
        submatrix
    }
}

////////////////////////////////////////////////////////////////////////////////
// Traits
////////////////////////////////////////////////////////////////////////////////

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
        for i in 0..self.ncols {
            for j in 0..self.nrows {
                if self.values[i][j] != other.values[i][j] {
                    return false;
                }
            }
        }
        true
    }
}

/// Addition ´+´ implementation for Matrix
impl<N: Copy> Add for Matrix<N> where N: Num {
    type Output = Matrix<N>;

    fn add(self, other: Matrix<N>) -> Matrix<N> {
        if (self.nrows != other.nrows) || (self.ncols != other.ncols) {
            panic!("sizes not compatible!");
        }

        let mut res: Matrix<N> = Matrix::<N>::init();
        for i in 0..self.ncols {

            let mut res_col: Vec<N> = Vec::new();
            for j in 0..self.nrows {
                let val = self.values[i][j] + other.values[i][j];
                res_col.push(val);
            }
            res.values.push(res_col);
        }
        res.ncols = self.ncols;
        res.nrows = self.nrows;

        res
    }
}

/// Substraction ´-´ implementation for Matrix
impl<N: Copy> Sub for Matrix<N> where N: Num {
    type Output = Matrix<N>;

    fn sub(self, other: Matrix<N>) -> Matrix<N> {
        if (self.nrows != other.nrows) || (self.ncols != other.ncols) {
            panic!("sizes not compatible!");
        }

        let mut res: Matrix<N> = Matrix::<N>::init();
        for i in 0..self.ncols {

            let mut res_col: Vec<N> = Vec::new();
            for j in 0..self.nrows {
                let val = self.values[i][j] - other.values[i][j];
                res_col.push(val);
            }
            res.values.push(res_col);
        }
        res.ncols = self.ncols;
        res.nrows = self.nrows;

        res
    }
}

/// Multiplication `*` implementation for &Matrix<N>
// Note: it will be good to implement Mul using parallelism
impl<'a, N: Copy + Default> Mul for &'a Matrix<N> where N: Num + Copy {
    type Output = Matrix<N>;

    fn mul(self, other: &'a Matrix<N>) -> Matrix<N> {
        if self.ncols != other.nrows {
            panic!("matrix dimension mismatch")
        } else {
            let mut res: Matrix<N> = Matrix::<N>::init();

            for ico in 0..other.ncols {
                let mut new_col: Vec<N> = Vec::new();
                for irs in 0..self.nrows {
                    let rs: Vec<N>;
                    let result_r = self.get_row(irs);
                    match result_r {
                        Some(x) => rs = x,
                        None => panic!("row of input matrix not found {}", irs),
                    }

                    let co: Vec<N>;
                    let result_c = other.get_col(ico);
                    match result_c {
                        Some(x) => co = x.clone(),
                        None => panic!("column of input matrix not found {}", irs),
                    }

                    let mut sum_r: N = N::default();
                    for el_rs in rs.iter() {
                        sum_r = sum_r + *el_rs;
                    }

                    let mut sum_c: N = N::default();
                    for el_co in co.iter() {
                        sum_c = sum_c + *el_co;
                    }

                    new_col.push(sum_r * sum_c);
                }
                res.push_col(new_col);
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
        for col in self.values.iter() {
            try!(write!(f, "[ "));
            for el in col.iter() {
                try!(write!(f, "{} ", el));
            }
            try!(writeln!(f, "]"));
        }
        try!(writeln!(f, "}}"));
        write!(f, "size: {col} x {row}", col = self.ncols, row = self.nrows)
    }
}
