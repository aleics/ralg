pub struct Matrix<N: Copy> {
    values: Vec<Vec<N>>,
    nrows: usize,
    ncols: usize,
}

pub fn init<N: Copy>() -> Matrix<N> {
    Matrix::<N> { values: Vec::with_capacity(0),
                  nrows: 0,
                  ncols: 0 }
}

pub fn init_with_capacity<N: Copy>(nr: usize, nc: usize) -> Matrix<N>{
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

    pub fn get_element(&self, i_col: usize, i_row: usize) -> &N {
        &self.values[i_col][i_row]
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
}
