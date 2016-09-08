extern crate ralg as r;

#[cfg(test)]
mod tests {
    use r::algebra::matrix::*;

    #[test]
    fn matrix_init_with_values_test() {
        let values: Vec<Vec<i32>> = vec![vec![1, 2], vec![3, 4]];
        let m = Matrix::<i32>::init_with_values(&values);

        assert_eq!(m.get_element(0, 0), values[0][0]);
        assert_eq!(m.get_element(0, 1), values[0][1]);
        assert_eq!(m.get_element(1, 0), values[1][0]);
        assert_eq!(m.get_element(1, 1), values[1][1]);
        assert_eq!(m.ncols(), values.len());
        assert_eq!(m.nrows(), values[0].len());
    }
    #[test]
    fn matrix_create_random_test() {
        let range: [f64; 2] = [0.0, 10.0];
        let m = Matrix::<f64>::create_random(3, 3, &range);

        assert_eq!(m.ncols(), 3);
        assert_eq!(m.nrows(), 3);
    }
    #[test]
    fn matrix_create_identity_test() {
        let m = Matrix::<f64>::create_identity(3);

        assert_eq!(m.get_element(0, 0), 1.0);
        assert_eq!(m.get_element(0, 1), 0.0);
        assert_eq!(m.get_element(0, 2), 0.0);
        assert_eq!(m.get_element(1, 0), 0.0);
        assert_eq!(m.get_element(1, 1), 1.0);
        assert_eq!(m.get_element(1, 2), 0.0);
        assert_eq!(m.get_element(2, 0), 0.0);
        assert_eq!(m.get_element(2, 1), 0.0);
        assert_eq!(m.get_element(2, 2), 1.0);
    }
    #[test]
    fn matrix_rows_test() {
        let m = Matrix::<u32>::init_with_capacity(5, 10);
        assert_eq!(m.nrows(), 10);
    }
    #[test]
    fn matrix_cols_test() {
        let m = Matrix::<u32>::init_with_capacity(5, 10);
        assert_eq!(m.ncols(), 5);
    }
    #[test]
    fn matrix_get_push_col_test() {
        let val: Vec<u32> = vec![1, 2, 3, 4, 5];
        let mut m = Matrix::<u32>::init_with_capacity(2, 5);

        m.push_col(val.clone());

        let result = m.get_col(0);
        let mut get_val: Vec<u32> = Vec::new();
        match result {
            Some(x) => get_val = x.clone(),
            None => assert_eq!(true, false),
        }
        assert_eq!(get_val.len(), val.len());
    }
    #[test]
    fn matrix_pop_col_test() {
        let val: Vec<u32> = vec![1, 2, 3, 4, 5];

        let mut m = Matrix::<u32>::init();
        for _ in 0..2 {
            m.push_col(val.clone());
        }

        m.pop_col(0);

        assert_eq!(1, m.ncols());
    }
    #[test]
    fn matrix_swap_col_test() {
        let val: Vec<u32> = vec![1, 2, 3, 4, 5];
        let val2: Vec<u32> = vec![6, 7, 8, 9, 10];

        let mut m = Matrix::<u32>::init();
        m.push_col(val.clone());
        m.push_col(val2.clone());

        m.swap_col(0, 1);
        assert_eq!(val[0], m.get_element(1, 0));
        assert_eq!(val2[0], m.get_element(0, 0));
    }
    #[test]
    fn matrix_swap_row_test() {
        let val: Vec<u32> = vec![1, 2, 3, 4, 5];
        let val2: Vec<u32> = vec![6, 7, 8, 9, 10];

        let mut m = Matrix::<u32>::init();
        m.push_col(val.clone());
        m.push_col(val2.clone());

        m.swap_row(0, 1);
        assert_eq!(val[1], m.get_element(0, 0));
        assert_eq!(val[0], m.get_element(0, 1));
    }
    #[test]
    fn matrix_get_row_test() {
        let resp: Vec<u32> = vec![1];
        let col: Vec<u32> = vec![1, 2, 3, 4, 5];
        let mut m = Matrix::<u32>::init_with_capacity(1, 5);
        m.push_col(col.clone());

        let result = m.get_row(0);
        let mut get_val: Vec<u32> = Vec::new();
        match result {
            Some(x) => get_val = x.clone(),
            None => assert_eq!(true, false),
        }
        assert_eq!(get_val.len(), resp.len());
        assert_eq!(get_val[0], resp[0]);
    }
    #[test]
    fn matrix_push_row_test() {
        let mut val: Vec<u32> = vec![1, 2, 3, 4, 5];
        let mut m = Matrix::<u32>::init();
        for _ in 0..2 {
            m.push_col(val.clone());
        }

        assert_eq!(m.nrows(), val.len());

        val = vec![6, 7];
        m.push_row(val.clone());
        assert_eq!(m.nrows(), 6);
    }
    #[test]
    fn matrix_pop_row_test() {
        let val: Vec<u32> = vec![1, 2, 3, 4, 5];
        let mut resp: Vec<u32> = val.clone();
        resp.remove(0);

        let mut m = Matrix::<u32>::init();
        m.push_col(val.clone());

        m.pop_row(0);

        assert_eq!(4, m.nrows());
        assert_eq!(resp[0], m.get_element(0, 0));
        assert_eq!(resp[1], m.get_element(0, 1));
        assert_eq!(resp[2], m.get_element(0, 2));
        assert_eq!(resp[3], m.get_element(0, 3));
    }
    #[test]
    fn matrix_get_element_test() {
        let col: Vec<u32> = vec![1, 2, 3, 4, 5];
        let mut m = Matrix::<u32>::init_with_capacity(1, 5);
        m.push_col(col);

        assert_eq!(m.get_element(0, 0), 1);
        assert_eq!(m.get_element(0, 1), 2);
        assert_eq!(m.get_element(0, 2), 3);
        assert_eq!(m.get_element(0, 3), 4);
        assert_eq!(m.get_element(0, 4), 5);
    }
    #[test]
    fn matrix_contains_test() {
        let col: Vec<f32> = vec![0.2, 2.5, 10.2, 8.7, 5.0];
        let mut m = Matrix::<f32>::init();
        m.push_col(col);

        let mut result = m.contains(&0.2);
        match result {
            Some(x) => assert_eq!(x, 0),
            None => assert_eq!(true, false),
        }

        result = m.contains(&0.7);
        match result {
            Some(_) => assert_eq!(true, false),
            None => assert_eq!(true, true),
        }
    }
    #[test]
    fn matrix_contains_col_test() {
        let col: Vec<i32> = vec![-1, 0, -2, 2, 3];
        let col2: Vec<i32> = vec![0, 0, -2, 2, 2];

        let mut m = Matrix::<i32>::init();
        m.push_col(col.clone());

        let mut result = m.contains_col(&col);
        match result {
            Some(x) => assert_eq!(x, 0),
            None => assert_eq!(true, false),
        }

        result = m.contains_col(&col2);
        match result {
            Some(_) => assert_eq!(true, false),
            None => assert_eq!(true, true),
        }
    }
    #[test]
    fn matrix_contains_row_test() {
        let col: Vec<i32> = vec![-1, 0, -2, 2, 3];
        let col2: Vec<i32> = vec![0, 0, -2, 2, 2];

        let mut m = Matrix::<i32>::init();
        m.push_col(col.clone());
        m.push_col(col2.clone());

        let row: Vec<i32> = vec![col[0], col2[0]];
        let row2: Vec<i32> = vec![2, 3];

        let mut result = m.contains_row(&row);
        match result {
            Some(x) => assert_eq!(x, 0),
            None => assert_eq!(true, false),
        }

        result = m.contains_col(&row2);
        match result {
            Some(_) => assert_eq!(true, false),
            None => assert_eq!(true, true),
        }
    }
    #[test]
    fn matrix_equal_to_test() {
        let col: Vec<i32> = vec![1, 3, 2, 3];
        let mut m = Matrix::<i32>::init();
        m.push_col(col);

        let eq = m.equal_to(&3);

        assert_eq!(eq.get_element(0, 0), false);
        assert_eq!(eq.get_element(0, 1), true);
        assert_eq!(eq.get_element(0, 2), false);
        assert_eq!(eq.get_element(0, 3), true);
    }
    #[test]
    fn matrix_equal_to_matrix_test() {
        let col: Vec<f32> = vec![1.0, 3.0, 2.0, 3.0];
        let mut m = Matrix::<f32>::init();
        m.push_col(col);

        let col2: Vec<f32> = vec![1.0, 0.5, 0.2, 3.0];
        let mut m2 = Matrix::<f32>::init();
        m2.push_col(col2);

        let eq_matrix = m.equal_to_matrix(&m2);

        assert_eq!(eq_matrix.get_element(0, 0), true);
        assert_eq!(eq_matrix.get_element(0, 1), false);
        assert_eq!(eq_matrix.get_element(0, 2), false);
        assert_eq!(eq_matrix.get_element(0, 3), true);
    }
    #[test]
    fn matrix_bigger_than_test() {
        let col: Vec<f32> = vec![0.1, 0.5, 0.8, 1.2];
        let mut m = Matrix::<f32>::init();
        m.push_col(col);

        let bigger = m.bigger_than(&0.9);

        assert_eq!(bigger.get_element(0, 0), false);
        assert_eq!(bigger.get_element(0, 1), false);
        assert_eq!(bigger.get_element(0, 2), false);
        assert_eq!(bigger.get_element(0, 3), true);
    }
    #[test]
    fn matrix_bigger_than_matrix_test() {
        let col: Vec<u32> = vec![1, 3, 2, 3];
        let mut m = Matrix::<u32>::init();
        m.push_col(col);

        let col2: Vec<u32> = vec![2, 0, 6, 3];
        let mut m2 = Matrix::<u32>::init();
        m2.push_col(col2);

        let eq_matrix = m.bigger_than_matrix(&m2);

        assert_eq!(eq_matrix.get_element(0, 0), false);
        assert_eq!(eq_matrix.get_element(0, 1), true);
        assert_eq!(eq_matrix.get_element(0, 2), false);
        assert_eq!(eq_matrix.get_element(0, 3), false);
    }
    #[test]
    fn matrix_eq_trait_test() {
        let col: Vec<u32> = vec![1, 3, 2, 3];
        let mut m = Matrix::<u32>::init();
        m.push_col(col);

        let m2 = m.clone();

        let col3: Vec<u32> = vec![1, 2, 3, 4];
        let mut m3 = Matrix::<u32>::init();
        m3.push_col(col3);

        assert_eq!(m == m2, true);
        assert_eq!(m == m3, false);
    }
    #[test]
    fn matrix_add_trait_test(){
        let col: Vec<u32> = vec![1, 3, 2, 3];
        let mut m = Matrix::<u32>::init();
        m.push_col(col);

        let m2: Matrix<u32> = m.clone();

        let res = m + m2;

        assert_eq!(res.get_element(0, 0), 2);
        assert_eq!(res.get_element(0, 1), 6);
        assert_eq!(res.get_element(0, 2), 4);
        assert_eq!(res.get_element(0, 3), 6);
        //assert_eq!(res.ncols(), m.ncols());
        //assert_eq!(res.nrows(), m.nrows());
    }
    #[test]
    fn matrix_sub_trait_test(){
        let col: Vec<u32> = vec![5, 6, 7, 8];
        let mut m = Matrix::<u32>::init();
        m.push_col(col);

        let col2: Vec<u32> = vec![1, 2, 3, 3];
        let mut m2 = Matrix::<u32>::init();
        m2.push_col(col2);

        let res = m - m2;

        assert_eq!(res.get_element(0, 0), 4);
        assert_eq!(res.get_element(0, 1), 4);
        assert_eq!(res.get_element(0, 2), 4);
        assert_eq!(res.get_element(0, 3), 5);
        //assert_eq!(res.ncols(), m.ncols());
        //assert_eq!(res.nrows(), m.nrows());
    }
    #[test]
    fn matrix_transpose_test(){
        let mut col: Vec<i32> = vec![1, 2, 3];
        let mut m = Matrix::<i32>::init();
        m.push_col(col);

        col = vec![5, 7, 8];
        m.push_col(col);

        let save = m.clone();

        m.transpose();

        assert_eq!(save.ncols(), m.nrows());
        assert_eq!(save.nrows(), m.ncols());
        assert_eq!(save.get_element(0, 0), m.get_element(0, 0));
        assert_eq!(save.get_element(0, 1), m.get_element(1, 0));
        assert_eq!(save.get_element(0, 2), m.get_element(2, 0));
    }
    #[test]
    fn matrix_scalar_mul_test() {
        let values: Vec<Vec<i32>> = vec![vec![1, 2, 3], vec![4, 5, 6]];
        let m = Matrix::<i32>::init_with_values(&values);
        let pr = m.scalar_mul(2);

        assert_eq!(pr.get_element(0, 0), 2);
        assert_eq!(pr.get_element(0, 1), 4);
        assert_eq!(pr.get_element(0, 2), 6);
        assert_eq!(pr.get_element(1, 0), 8);
        assert_eq!(pr.get_element(1, 1), 10);
        assert_eq!(pr.get_element(1, 2), 12);
    }
    #[test]
    fn matrix_mul_trait_test(){
        let mut values: Vec<Vec<i32>> = vec![vec![1, 4], vec![2, 5], vec![3, 6]];
        let m = Matrix::<i32>::init_with_values(&values);

        values = vec![vec![7, 8, 9], vec![10, 11, 12]];
        let m2 = Matrix::<i32>::init_with_values(&values);

        let prod = &m * &m2;

        assert_eq!(prod.ncols(), m2.ncols());
        assert_eq!(prod.nrows(), m.nrows());
    }
    #[test]
    fn matrix_get_diagonal_test() {
        let values: Vec<Vec<i64>> = vec![vec![1, -2, 2], vec![4, -5, 6], vec![2, 1, -2]];
        let m = Matrix::<i64>::init_with_values(&values);

        let d: Matrix<i64> = m.get_diagonal();

        assert_eq!(d.ncols(), m.ncols());
        assert_eq!(d.nrows(), m.nrows());
        assert_eq!(d.get_element(0, 0), m.get_element(0, 0));
        assert_eq!(d.get_element(0, 1), 0);
        assert_eq!(d.get_element(0, 2), 0);
        assert_eq!(d.get_element(1, 1), m.get_element(1, 1));
        assert_eq!(d.get_element(1, 0), 0);
        assert_eq!(d.get_element(1, 2), 0);
        assert_eq!(d.get_element(2, 2), m.get_element(2, 2));
        assert_eq!(d.get_element(2, 0), 0);
        assert_eq!(d.get_element(2, 1), 0);
    }
    #[test]
    fn matrix_submatrix_test() {
        let values: Vec<Vec<f32>> = vec![vec![1.0, 4.0], vec![2.0, 5.0], vec![3.0, 6.0]];
        let m = Matrix::<f32>::init_with_values(&values);

        let range_col: [usize; 2] = [1, 2];
        let range_row: [usize; 2] = [0, 0];

        let s = m.submatrix(&range_col, &range_row);

        assert_eq!(s.ncols(), (range_col[1] + 1) - range_col[0]);
        assert_eq!(s.nrows(), (range_row[1] + 1) - range_row[0]);
    }
}
