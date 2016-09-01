#[derive(Debug, Clone)]
pub struct Matrix<N> {
    values: Vec<Vec<N>>,
    nrows: usize,
    ncols: usize,
}

pub fn init<N>() -> Matrix<N> {
    Matrix::<N> { values: Vec::with_capacity(0),
                  nrows: 0,
                  ncols: 0 }
}

pub fn init_dim<N>(nr: usize, nc: usize) -> Matrix<N>{
    let mut vec: Vec<Vec<N>> = Vec::with_capacity(nc);
    for i in 0..vec.len() {
        vec[i] = Vec::with_capacity(nr);
    }
    Matrix::<N> { values: vec,
                  nrows: nr,
                  ncols: nc }
}

impl<N: Clone> Matrix<N> {
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

    pub fn get_row(&self, index: usize) -> Option<Vec<&N>> {
        let mut ret: Vec<&N> = Vec::new();
        for i in 0..self.values.len() {
            ret.push(&self.values[i][index]);
        }
        Some(ret)
    }

    pub fn get_element(&self, i_col: usize, i_row: usize) -> &N {
        &self.values[i_col][i_row]
    }

    pub fn push_col(&mut self, row: Vec<N>) {
        self.values.push(row);
    }
}
