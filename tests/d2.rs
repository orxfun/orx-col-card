use alloc::collections::VecDeque;
use alloc::vec;
use alloc::vec::Vec;
use orx_col_card::*;

extern crate alloc;

fn assert_d2<C: Card<D2>>(col: &C, expected_row_lens: &[usize]) {
    assert_eq!(col.card([]), expected_row_lens.len());
    assert_eq!(col.try_card([]), Some(expected_row_lens.len()));

    for (i, &expected_len) in expected_row_lens.iter().enumerate() {
        assert_eq!(col.card(i), expected_len);
        assert_eq!(col.card([i]), expected_len);
        assert_eq!(col.try_card(i), Some(expected_len));
        assert_eq!(col.try_card([i]), Some(expected_len));
    }

    let out_of_bounds = expected_row_lens.len();
    assert_eq!(col.try_card(out_of_bounds), None);
    assert_eq!(col.try_card([out_of_bounds]), None);
    assert_eq!(col.try_card(out_of_bounds + 10), None);
    assert_eq!(col.try_card([out_of_bounds + 10]), None);
}

#[test]
fn vec_of_vec_d2() {
    let empty: Vec<Vec<i32>> = vec![];
    assert_d2(&empty, &[]);

    let matrix = vec![vec![1, 2, 3], vec![4], vec![], vec![5, 6]];
    assert_d2(&matrix, &[3, 1, 0, 2]);
    assert_d2(&&matrix, &[3, 1, 0, 2]);
}

#[test]
fn vec_deque_of_vec_deque_d2() {
    let deque: VecDeque<VecDeque<i32>> = VecDeque::from([
        VecDeque::from([1, 2]),
        VecDeque::from([3, 4, 5, 6]),
        VecDeque::new(),
    ]);
    assert_d2(&deque, &[2, 4, 0]);
    assert_d2(&&deque, &[2, 4, 0]);
}

#[test]
fn slice_of_slice_d2() {
    let row0: &[i32] = &[1, 2];
    let row1: &[i32] = &[3, 4, 5];
    let row2: &[i32] = &[];
    let matrix: &[&[i32]] = &[row0, row1, row2];

    assert_d2(&matrix, &[2, 3, 0]);
    assert_d2(&&matrix, &[2, 3, 0]);
}

#[test]
fn mixed_types_d2() {
    // Vec of VecDeque
    let v_vd: Vec<VecDeque<i32>> = vec![
        VecDeque::from([1, 2, 3]),
        VecDeque::new(),
        VecDeque::from([4, 5]),
    ];
    assert_d2(&v_vd, &[3, 0, 2]);

    // Vec of &[T]
    let s0: &[i32] = &[1];
    let s1: &[i32] = &[2, 3, 4];
    let v_s: Vec<&[i32]> = vec![s0, s1];
    assert_d2(&v_s, &[1, 3]);

    // VecDeque of Vec
    let vd_v: VecDeque<Vec<i32>> = VecDeque::from([vec![10, 20], vec![], vec![30, 40, 50, 60]]);
    assert_d2(&vd_v, &[2, 0, 4]);

    // VecDeque of &[T]
    let vd_s: VecDeque<&[i32]> = VecDeque::from([s1, s0]);
    assert_d2(&vd_s, &[3, 1]);

    // Slice of Vec
    let v0 = vec![1, 2, 3, 4];
    let v1 = vec![5];
    let s_v: &[Vec<i32>] = &[v0, v1];
    assert_d2(&s_v, &[4, 1]);

    // Slice of VecDeque
    let vd0 = VecDeque::from([1, 2]);
    let vd1 = VecDeque::from([3, 4, 5]);
    let s_vd: &[VecDeque<i32>] = &[vd0, vd1];
    assert_d2(&s_vd, &[2, 3]);
}

#[test]
#[should_panic]
fn vec_d2_out_of_bounds_panics() {
    let matrix = vec![vec![1, 2], vec![3]];
    let _ = Card::<D2>::card(&matrix, 2);
}
