extern crate ralg as r;

use r::algebra::matrix::*;

#[test]
fn rows_test() {
    let m = init_with_capacity::<u32>(10, 5);
    assert_eq!(m.nrows(), 10);
}
#[test]
fn cols_test() {
    let m = init_with_capacity::<u32>(10, 5);
    assert_eq!(m.ncols(), 5);
}
#[test]
fn get_col_test() {
    let val: Vec<u32> = vec![1, 2, 3, 4, 5];
    let mut m = init_with_capacity::<u32>(5, 2);
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
    let mut m = init_with_capacity::<u32>(5, 1);
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
fn get_element_test() {
    let mut col: Vec<u32> = vec![1, 2, 3, 4, 5];
    let mut m = init_with_capacity::<u32>(5, 1);
    m.push_col(col);

    assert_eq!(*m.get_element(0, 0), 1);
    assert_eq!(*m.get_element(0, 1), 2);
    assert_eq!(*m.get_element(0, 2), 3);
    assert_eq!(*m.get_element(0, 3), 4);
    assert_eq!(*m.get_element(0, 4), 5);
}
