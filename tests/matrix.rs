extern crate ralg as r;

use r::algebra::matrix::*;

#[test]
fn rows_test() {
    let m = init_with_capacity::<u32>(5, 10);
    assert_eq!(m.nrows(), 10);
}
#[test]
fn cols_test() {
    let m = init_with_capacity::<u32>(5, 10);
    assert_eq!(m.ncols(), 5);
}
#[test]
fn get_col_test() {
    let val: Vec<u32> = vec![1, 2, 3, 4, 5];
    let mut m = init_with_capacity::<u32>(2, 5);
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
fn get_row_test() {
    let resp: Vec<u32> = vec![1];
    let col: Vec<u32> = vec![1, 2, 3, 4, 5];
    let mut m = init_with_capacity::<u32>(1, 5);
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
fn push_row_test() {
    let mut val: Vec<u32> = vec![1, 2, 3, 4, 5];
    let mut m = init::<u32>();
    for _ in 0..2 {
        m.push_col(val.clone());
    }
    assert_eq!(m.nrows(), val.len());

    val = vec![6, 7];
    m.push_row(val.clone());
    assert_eq!(m.nrows(), 6);
}
#[test]
fn get_element_test() {
    let col: Vec<u32> = vec![1, 2, 3, 4, 5];
    let mut m = init_with_capacity::<u32>(1, 5);
    m.push_col(col);

    assert_eq!(m.get_element(0, 0), 1);
    assert_eq!(m.get_element(0, 1), 2);
    assert_eq!(m.get_element(0, 2), 3);
    assert_eq!(m.get_element(0, 3), 4);
    assert_eq!(m.get_element(0, 4), 5);
}
#[test]
fn equal_to_test() {
    let col: Vec<i32> = vec![1, 3, 2, 3];
    let mut m = init::<i32>();
    m.push_col(col);

    let eq = m.equal_to(3);

    assert_eq!(eq.get_element(0, 0), false);
    assert_eq!(eq.get_element(0, 1), true);
    assert_eq!(eq.get_element(0, 2), false);
    assert_eq!(eq.get_element(0, 3), true);
}
#[test]
fn equal_to_matrix_test() {
    let col: Vec<f32> = vec![1.0, 3.0, 2.0, 3.0];
    let mut m = init::<f32>();
    m.push_col(col);

    let col2: Vec<f32> = vec![1.0, 0.5, 0.2, 3.0];
    let mut m2 = init::<f32>();
    m2.push_col(col2);

    let eq_matrix = m.equal_to_matrix(&m2);

    assert_eq!(eq_matrix.get_element(0, 0), true);
    assert_eq!(eq_matrix.get_element(0, 1), false);
    assert_eq!(eq_matrix.get_element(0, 2), false);
    assert_eq!(eq_matrix.get_element(0, 3), true);
}
#[test]
fn bigger_than_test() {
    let col: Vec<f32> = vec![0.1, 0.5, 0.8, 1.2];
    let mut m = init::<f32>();
    m.push_col(col);

    let bigger = m.bigger_than(0.9);

    assert_eq!(bigger.get_element(0, 0), false);
    assert_eq!(bigger.get_element(0, 1), false);
    assert_eq!(bigger.get_element(0, 2), false);
    assert_eq!(bigger.get_element(0, 3), true);
}
#[test]
fn bigger_than_matrix_test() {
    let col: Vec<u32> = vec![1, 3, 2, 3];
    let mut m = init::<u32>();
    m.push_col(col);

    let col2: Vec<u32> = vec![2, 0, 6, 3];
    let mut m2 = init::<u32>();
    m2.push_col(col2);

    let eq_matrix = m.bigger_than_matrix(&m2);

    assert_eq!(eq_matrix.get_element(0, 0), false);
    assert_eq!(eq_matrix.get_element(0, 1), true);
    assert_eq!(eq_matrix.get_element(0, 2), false);
    assert_eq!(eq_matrix.get_element(0, 3), false);
}
