extern crate rsmath as r;

#[cfg(test)]
mod tests {
    use r::algebra::matrix::*;
    use r::algebra::vector::*;

    // --------------- Matrix TEST ----------------------------------------

    #[test]
    fn matrix_init_test() {
        let values: Vec<Vec<i32>> = vec![vec![1, 2], vec![3, 4]];
        let m = Matrix::<i32>::init(&values);

        assert_eq!(m.get_element(0, 0), values[0][0]);
        assert_eq!(m.get_element(1, 0), values[1][0]);
        assert_eq!(m.get_element(0, 1), values[0][1]);
        assert_eq!(m.get_element(1, 1), values[1][1]);
        assert_eq!(m.nrows(), values.len());
        assert_eq!(m.ncols(), values[0].len());
    }
    #[test]
    fn matrix_random_test() {
        let range: [f64; 2] = [0.0, 10.0];
        let m = Matrix::<f64>::random(2, 3, &range);

        assert_eq!(m.nrows(), 2);
        assert_eq!(m.ncols(), 3);
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
        assert_eq!(m.nrows(), 5);
    }
    #[test]
    fn matrix_cols_test() {
        let m = Matrix::<u32>::init_with_capacity(5, 10);
        assert_eq!(m.ncols(), 10);
    }
    #[test]
    fn matrix_set_row_test() {
        let v: Vec<i32> = vec![-1, 2];
        let mut m = Matrix::<i32>::init(&vec![vec![1, 3], vec![2, 5], vec![3, 6]]);

        m.set_row(0, &v);

        assert_eq!(m.row(0).unwrap().clone(), v);
    }
    #[test]
    fn matrix_get_push_row_test() {
        let val: Vec<u32> = vec![1, 2, 3, 4, 5];
        let mut m = Matrix::<u32>::init_with_capacity(5, 2);

        m.push_row(val.clone());

        let result = m.row(0);
        let mut get_val: Vec<u32> = Vec::new();
        match result {
            Some(x) => get_val = x.clone(),
            None => assert_eq!(true, false),
        }
        assert_eq!(get_val.len(), val.len());
    }
    #[test]
    fn matrix_pop_row_test() {
        let val: Vec<u32> = vec![1, 2, 3, 4, 5];

        let mut m = Matrix::<u32>::new();
        for _ in 0..2 {
            m.push_row(val.clone());
        }

        m.pop_row(0);

        assert_eq!(1, m.nrows());
    }
    #[test]
    fn matrix_swap_row_test() {
        let val: Vec<u32> = vec![1, 2, 3, 4, 5];
        let val2: Vec<u32> = vec![6, 7, 8, 9, 10];

        let mut m = Matrix::<u32>::new();
        m.push_row(val.clone());
        m.push_row(val2.clone());

        m.swap_row(0, 1);
        assert_eq!(val[0], m.get_element(1, 0));
        assert_eq!(val2[0], m.get_element(0, 0));
    }
    #[test]
    fn matrix_swap_col_test() {
        let val: Vec<u32> = vec![1, 2, 3, 4, 5];
        let val2: Vec<u32> = vec![6, 7, 8, 9, 10];

        let mut m = Matrix::<u32>::new();
        m.push_row(val.clone());
        m.push_row(val2.clone());

        m.swap_col(0, 1);
        assert_eq!(val[1], m.get_element(0, 0));
        assert_eq!(val[0], m.get_element(0, 1));
    }
    #[test]
    fn matrix_col_test() {
        let resp: Vec<u32> = vec![1];
        let row: Vec<u32> = vec![1, 2, 3, 4, 5];
        let mut m = Matrix::<u32>::init_with_capacity(1, 5);
        m.push_row(row.clone());

        let result = m.col(0);
        let mut get_val: Vec<u32> = Vec::new();
        match result {
            Some(x) => get_val = x.clone(),
            None => assert_eq!(true, false),
        }
        assert_eq!(get_val.len(), resp.len());
        assert_eq!(get_val[0], resp[0]);
    }
    #[test]
    fn matrix_set_col_test() {
        let v: Vec<i32> = vec![-1, 2, 4];
        let mut m = Matrix::<i32>::init(&vec![vec![1, 3], vec![2, 5], vec![3, 6]]);

        m.set_col(0, &v);

        assert_eq!(m.col(0).unwrap().clone(), v);
    }
    #[test]
    fn matrix_push_col_test() {
        let mut val: Vec<u32> = vec![1, 2, 3, 4, 5];
        let mut m = Matrix::<u32>::new();
        for _ in 0..2 {
            m.push_row(val.clone());
        }

        assert_eq!(m.ncols(), val.len());

        val = vec![6, 7];
        m.push_col(val.clone());
        assert_eq!(m.ncols(), 6);
    }
    #[test]
    fn matrix_pop_col_test() {
        let val: Vec<u32> = vec![1, 2, 3, 4, 5];
        let mut resp: Vec<u32> = val.clone();
        resp.remove(0);

        let mut m = Matrix::<u32>::new();
        m.push_row(val.clone());

        m.pop_col(0);

        assert_eq!(4, m.ncols());
        assert_eq!(resp[0], m.get_element(0, 0));
        assert_eq!(resp[1], m.get_element(0, 1));
        assert_eq!(resp[2], m.get_element(0, 2));
        assert_eq!(resp[3], m.get_element(0, 3));
    }
    #[test]
    fn matrix_get_element_test() {
        let row: Vec<u32> = vec![1, 2, 3, 4, 5];
        let mut m = Matrix::<u32>::init_with_capacity(1, 5);
        m.push_row(row);

        assert_eq!(m.get_element(0, 0), 1);
        assert_eq!(m.get_element(0, 1), 2);
        assert_eq!(m.get_element(0, 2), 3);
        assert_eq!(m.get_element(0, 3), 4);
        assert_eq!(m.get_element(0, 4), 5);
    }
    #[test]
    fn matrix_contains_test() {
        let row: Vec<f32> = vec![0.2, 2.5, 10.2, 8.7, 5.0];
        let mut m = Matrix::<f32>::new();
        m.push_row(row);

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
    fn matrix_contains_row_test() {
        let row: Vec<i32> = vec![-1, 0, -2, 2, 3];
        let row2: Vec<i32> = vec![0, 0, -2, 2, 2];

        let mut m = Matrix::<i32>::new();
        m.push_row(row.clone());

        let mut result = m.contains_row(&row);
        match result {
            Some(x) => assert_eq!(x, 0),
            None => assert_eq!(true, false),
        }

        result = m.contains_row(&row2);
        match result {
            Some(_) => assert_eq!(true, false),
            None => assert_eq!(true, true),
        }
    }
    #[test]
    fn matrix_contains_col_test() {
        let row: Vec<i32> = vec![-1, 0, -2, 2, 3];
        let row2: Vec<i32> = vec![0, 0, -2, 2, 2];

        let mut m = Matrix::<i32>::new();
        m.push_row(row.clone());
        m.push_row(row2.clone());

        let col: Vec<i32> = vec![row[0], row2[0]];
        let col2: Vec<i32> = vec![2, 3];

        let mut result = m.contains_col(&col);
        match result {
            Some(x) => assert_eq!(x, 0),
            None => assert_eq!(true, false),
        }

        result = m.contains_row(&col2);
        match result {
            Some(_) => assert_eq!(true, false),
            None => assert_eq!(true, true),
        }
    }
    #[test]
    fn matrix_equal_to_test() {
        let row: Vec<i32> = vec![1, 3, 2, 3];
        let mut m = Matrix::<i32>::new();
        m.push_row(row);

        let eq = m.equal_to(&3);

        assert_eq!(eq.get_element(0, 0), false);
        assert_eq!(eq.get_element(0, 1), true);
        assert_eq!(eq.get_element(0, 2), false);
        assert_eq!(eq.get_element(0, 3), true);
    }
    #[test]
    fn matrix_equal_to_matrix_test() {
        let row: Vec<f32> = vec![1.0, 3.0, 2.0, 3.0];
        let mut m = Matrix::<f32>::new();
        m.push_row(row);

        let row2: Vec<f32> = vec![1.0, 0.5, 0.2, 3.0];
        let mut m2 = Matrix::<f32>::new();
        m2.push_row(row2);

        let eq_matrix = m.equal_to_matrix(&m2);

        assert_eq!(eq_matrix.get_element(0, 0), true);
        assert_eq!(eq_matrix.get_element(0, 1), false);
        assert_eq!(eq_matrix.get_element(0, 2), false);
        assert_eq!(eq_matrix.get_element(0, 3), true);
    }
    #[test]
    fn matrix_bigger_than_test() {
        let row: Vec<f32> = vec![0.1, 0.5, 0.8, 1.2];
        let mut m = Matrix::<f32>::new();
        m.push_row(row);

        let bigger = m.bigger_than(&0.9);

        assert_eq!(bigger.get_element(0, 0), false);
        assert_eq!(bigger.get_element(0, 1), false);
        assert_eq!(bigger.get_element(0, 2), false);
        assert_eq!(bigger.get_element(0, 3), true);
    }
    #[test]
    fn matrix_bigger_than_matrix_test() {
        let row: Vec<u32> = vec![1, 3, 2, 3];
        let mut m = Matrix::<u32>::new();
        m.push_row(row);

        let row2: Vec<u32> = vec![2, 0, 6, 3];
        let mut m2 = Matrix::<u32>::new();
        m2.push_row(row2);

        let eq_matrix = m.bigger_than_matrix(&m2);

        assert_eq!(eq_matrix.get_element(0, 0), false);
        assert_eq!(eq_matrix.get_element(0, 1), true);
        assert_eq!(eq_matrix.get_element(0, 2), false);
        assert_eq!(eq_matrix.get_element(0, 3), false);
    }
    #[test]
    fn matrix_eq_trait_test() {
        let row: Vec<u32> = vec![1, 3, 2, 3];
        let mut m = Matrix::<u32>::new();
        m.push_row(row);

        let m2 = m.clone();

        let row3: Vec<u32> = vec![1, 2, 3, 4];
        let mut m3 = Matrix::<u32>::new();
        m3.push_row(row3);

        assert_eq!(m == m2, true);
        assert_eq!(m == m3, false);
    }
    #[test]
    fn matrix_add_trait_test(){
        let row: Vec<u32> = vec![1, 3, 2, 3];
        let mut m = Matrix::<u32>::new();
        m.push_row(row);

        let m2: Matrix<u32> = m.clone();

        let res = &m + &m2;

        assert_eq!(res.get_element(0, 0), 2);
        assert_eq!(res.get_element(0, 1), 6);
        assert_eq!(res.get_element(0, 2), 4);
        assert_eq!(res.get_element(0, 3), 6);
        assert_eq!(res.ncols(), m.ncols());
        assert_eq!(res.nrows(), m2.nrows());
    }
    #[test]
    fn matrix_sub_trait_test(){
        let row: Vec<u32> = vec![5, 6, 7, 8];
        let mut m = Matrix::<u32>::new();
        m.push_row(row);

        let row2: Vec<u32> = vec![1, 2, 3, 3];
        let mut m2 = Matrix::<u32>::new();
        m2.push_row(row2);

        let res = &m - &m2;

        assert_eq!(res.get_element(0, 0), 4);
        assert_eq!(res.get_element(0, 1), 4);
        assert_eq!(res.get_element(0, 2), 4);
        assert_eq!(res.get_element(0, 3), 5);
        assert_eq!(res.ncols(), m.ncols());
        assert_eq!(res.nrows(), m.nrows());
    }
    #[test]
    fn matrix_transpose_test(){
        let mut row: Vec<i32> = vec![1, 2, 3];
        let mut m = Matrix::<i32>::new();
        m.push_row(row);

        row = vec![5, 7, 8];
        m.push_row(row);

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
        let m = Matrix::<i32>::init(&values);
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
        let m = Matrix::<i32>::init(&values);

        values = vec![vec![7, 8, 9], vec![10, 11, 12]];
        let m2 = Matrix::<i32>::init(&values);

        let prod = &m * &m2;

        assert_eq!(prod.ncols(), m.nrows());
        assert_eq!(prod.nrows(), m2.ncols());
    }
    #[test]
    fn matrix_get_diagonal_test() {
        let values: Vec<Vec<i64>> = vec![vec![1, -2, 2], vec![4, -5, 6], vec![2, 1, -2]];
        let m = Matrix::<i64>::init(&values);

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
        let m = Matrix::<f32>::init(&values);

        let range_row: [usize; 2] = [1, 2];
        let range_col: [usize; 2] = [0, 0];

        let s = m.submatrix(&range_row, &range_col);

        assert_eq!(s.ncols(), (range_col[1] + 1) - range_col[0]);
        assert_eq!(s.nrows(), (range_row[1] + 1) - range_row[0]);
    }
    #[test]
    fn matrix_eucl_distance_col_test() {
        let m = Matrix::<i32>::init(&vec![vec![1, 3], vec![2, 5], vec![3, 6]]);

        let d = m.eucl_distance_row();

        assert_eq!(d.ncols(), m.nrows());
        assert_eq!(d.nrows(), m.nrows());
    }
    #[test]
    fn matrix_row_iter_at_test() {
        let m = Matrix::<i32>::init(&vec![vec![1, 3], vec![2, 5], vec![3, 6]]);
        let mut i = m.row_iter_at(1);

        assert_eq!(i.next().unwrap(), &vec![2, 5]);
        assert_eq!(i.next().unwrap(), &vec![3, 6]);
    }
    #[test]
    fn matrix_row_iter_test() {
        let m = Matrix::<i32>::init(&vec![vec![1, 3], vec![2, 5], vec![3, 6]]);
        let mut i = m.row_iter();

        assert_eq!(i.next().unwrap(), &vec![1, 3]);
        assert_eq!(i.next().unwrap(), &vec![2, 5]);
        assert_eq!(i.next().unwrap(), &vec![3, 6]);
    }
    #[test]
    fn matrix_col_iter_at_test() {
        let m = Matrix::<i32>::init(&vec![vec![1, 3], vec![2, 5], vec![3, 6]]);
        let mut i = m.col_iter_at(1);

        assert_eq!(i.next().unwrap(), vec![3, 5, 6]);
    }
    #[test]
    fn matrix_col_iter_test() {
        let m = Matrix::<i32>::init(&vec![vec![1, 3], vec![2, 5], vec![3, 6]]);
        let mut i = m.col_iter();

        assert_eq!(i.next().unwrap(), vec![1, 2, 3]);
        assert_eq!(i.next().unwrap(), vec![3, 5, 6]);
    }
    #[test]
    fn matrix_el_iter_at_test() {
        let m = Matrix::<i32>::init(&vec![vec![1, 3], vec![2, 5], vec![3, 6]]);
        let mut i = m.el_iter_at(1);

        assert_eq!(i.next().unwrap(), 3);
        assert_eq!(i.next().unwrap(), 2);
        assert_eq!(i.next().unwrap(), 5);
        assert_eq!(i.next().unwrap(), 3);
        assert_eq!(i.next().unwrap(), 6);
    }
    #[test]
    fn matrix_el_iter_test() {
        let m = Matrix::<i32>::init(&vec![vec![8, 3], vec![5, 10]]);
        let mut i = m.el_iter();

        assert_eq!(i.next().unwrap(), m.get_element(0, 0));
        assert_eq!(i.next().unwrap(), m.get_element(0, 1));
        assert_eq!(i.next().unwrap(), m.get_element(1, 0));
        assert_eq!(i.next().unwrap(), m.get_element(1, 1));
    }

    // --------------- Vector TEST ----------------------------------------
    #[test]
    fn vector_init_test() {
        let v = Vector::<f64>::init(&vec![2.5f64, -2.1f64, 5.3f64]);

        assert_eq!(v.size(), 3);
        assert_eq!(v.el(0), 2.5f64);
        assert_eq!(v.el(1), -2.1f64);
        assert_eq!(v.el(2), 5.3f64);
    }
    #[test]
    fn vector_zeros_test() {
        let v = Vector::<i32>::zeros(2);

        assert_eq!(v.size(), 2);
        assert_eq!(v.el(0), 0);
        assert_eq!(v.el(1), 0);
    }
    #[test]
    fn vector_ones_test() {
        let v = Vector::<u32>::ones(2);

        assert_eq!(v.size(), 2);
        assert_eq!(v.el(0), 1);
        assert_eq!(v.el(1), 1);
    }
    #[test]
    fn vector_random_test() {
        let v = Vector::<i32>::random(5, &[-1i32, 2i32]);

        assert_eq!(v.size(), 5);
        assert_eq!(v.el(0) >= -1i32 && v.el(0) <= 2i32, true);
        assert_eq!(v.el(1) >= -1i32 && v.el(1) <= 2i32, true);
        assert_eq!(v.el(2) >= -1i32 && v.el(2) <= 2i32, true);
        assert_eq!(v.el(3) >= -1i32 && v.el(3) <= 2i32, true);
        assert_eq!(v.el(4) >= -1i32 && v.el(4) <= 2i32, true);
    }
    #[test]
    fn vector_push_test() {
        let mut v = Vector::<f32>::new();
        v.push(1f32);

        assert_eq!(v.size(), 1);
        assert_eq!(v.el(0), 1f32);
    }
    #[test]
    fn vector_remove_test() {
        let mut v = Vector::<f32>::init(&vec![2.0, 3.1, 2.1]);
        v.remove(1);

        assert_eq!(v.size(), 2);
        assert_eq!(v.el(0), 2.0);
        assert_eq!(v.el(1), 2.1);
    }
    #[test]
    fn vector_pop_test() {
        let mut v = Vector::<f64>::init(&vec![2.0, 3.1, 2.1]);
        v.pop();

        assert_eq!(v.size(), 2);
        assert_eq!(v.el(0), 2.0);
        assert_eq!(v.el(1), 3.1);
    }
    #[test]
    fn vector_swap_test() {
        let mut v = Vector::<i64>::init(&vec![2, -1, 1]);
        v.swap(0, 1);

        assert_eq!(v.size(), 3);
        assert_eq!(v.el(0), -1);
        assert_eq!(v.el(1), 2);
        assert_eq!(v.el(2), 1);
    }
    #[test]
    fn vector_append_test() {
        let mut v1 = Vector::<i64>::init(&vec![2, -1, 1]);
        let v2 = Vector::<i64>::init(&vec![5, -7, 10]);

        v1.append(&v2);

        assert_eq!(v1.size(), 6);
        assert_eq!(v2.size(), 3);
    }
    #[test]
    fn vector_clear_test() {
        let mut v = Vector::<f32>::init(&vec![2.0, 3.1, 2.1]);
        v.clear();

        assert_eq!(v.size(), 0);
    }
    #[test]
    fn vector_sort_min_test() {
        let mut v = Vector::<i64>::init(&vec![2, -1, 1]);
        v.sort_min();

        assert_eq!(v.el(0), -1);
        assert_eq!(v.el(1), 1);
        assert_eq!(v.el(2), 2);
    }
    #[test]
    fn vector_sort_max_test() {
        let mut v = Vector::<i64>::init(&vec![2, -1, 1]);
        v.sort_max();

        assert_eq!(v.el(0), 2);
        assert_eq!(v.el(1), 1);
    }
    #[test]
    fn vector_set_el_test() {
        let mut v = Vector::<i64>::init(&vec![2, -1, 1]);
        v.set_el(2, 5i64);

        assert_eq!(v.el(2), 5i64);
    }
    #[test]
    fn vector_max_test() {
        let v = Vector::<i64>::init(&vec![2, -1, 1]);
        let (max, idx_max) = v.max();

        assert_eq!(max, 2i64);
        assert_eq!(idx_max, 0usize);
    }
    #[test]
    fn vector_min_test() {
        let v = Vector::<i64>::init(&vec![2, -1, 1]);
        let (min, idx_min) = v.min();

        assert_eq!(min, -1i64);
        assert_eq!(idx_min, 1usize);
    }
    #[test]
    fn vector_median_test() {
        let v = Vector::<i64>::init(&vec![2, 4, 0, 6]);
        let median = v.median();

        assert_eq!(median, 3f64);
    }
}
