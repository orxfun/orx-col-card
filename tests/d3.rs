use alloc::collections::VecDeque;
use alloc::vec;
use alloc::vec::Vec;
use orx_col_card::*;

extern crate alloc;

fn assert_d3<C: Card<D3>>(
    col: &C,
    expected_l0: usize,
    expected_l1: &[usize],
    expected_l2: &[&[usize]],
) {
    assert_eq!(col.card([]), expected_l0);
    assert_eq!(col.try_card([]), Some(expected_l0));

    for (i, &l1) in expected_l1.iter().enumerate() {
        assert_eq!(col.card(i), l1);
        assert_eq!(col.card([i]), l1);
        assert_eq!(col.try_card(i), Some(l1));
        assert_eq!(col.try_card([i]), Some(l1));
    }
    assert_eq!(col.try_card(expected_l1.len()), None);
    assert_eq!(col.try_card([expected_l1.len()]), None);

    for (i, row) in expected_l2.iter().enumerate() {
        for (j, &l2) in row.iter().enumerate() {
            assert_eq!(col.card([i, j]), l2);
            assert_eq!(col.try_card([i, j]), Some(l2));
        }
        assert_eq!(col.try_card([i, row.len()]), None);
    }
    assert_eq!(col.try_card([expected_l2.len(), 0]), None);
    assert_eq!(col.try_card([100, 100]), None);
}

#[test]
fn vec_of_vec_of_vec_d3() {
    let empty: Vec<Vec<Vec<i32>>> = vec![];
    assert_d3(&empty, 0, &[], &[]);

    let tensor = vec![
        vec![vec![1, 2, 3], vec![4]],
        vec![vec![], vec![5, 6], vec![7, 8, 9, 10]],
    ];

    assert_d3(&tensor, 2, &[2, 3], &[&[3, 1], &[0, 2, 4]]);
    assert_d3(&&tensor, 2, &[2, 3], &[&[3, 1], &[0, 2, 4]]);

    // Direct calls with Card::<D3>
    assert_eq!(Card::<D3>::card(&tensor, []), 2);
    assert_eq!(Card::<D3>::card(&tensor, 0), 2);
    assert_eq!(Card::<D3>::card(&tensor, [0]), 2);
    assert_eq!(Card::<D3>::card(&tensor, [0, 0]), 3);
    assert_eq!(Card::<D3>::card(&tensor, [1, 2]), 4);

    assert_eq!(Card::<D3>::try_card(&tensor, []), Some(2));
    assert_eq!(Card::<D3>::try_card(&tensor, 1), Some(3));
    assert_eq!(Card::<D3>::try_card(&tensor, [1]), Some(3));
    assert_eq!(Card::<D3>::try_card(&tensor, [1, 2]), Some(4));
    assert_eq!(Card::<D3>::try_card(&tensor, [1, 3]), None);
    assert_eq!(Card::<D3>::try_card(&tensor, [2, 0]), None);
}

#[test]
fn vec_deque_of_vec_deque_of_vec_deque_d3() {
    let d3: VecDeque<VecDeque<VecDeque<i32>>> = VecDeque::from([
        VecDeque::from([VecDeque::from([1, 2]), VecDeque::from([3])]),
        VecDeque::from([VecDeque::new()]),
    ]);

    assert_d3(&d3, 2, &[2, 1], &[&[2, 1], &[0]]);
    assert_d3(&&d3, 2, &[2, 1], &[&[2, 1], &[0]]);
}

#[test]
fn slice_of_slice_of_slice_d3() {
    let r0: &[i32] = &[1, 2, 3];
    let r1: &[i32] = &[4];
    let m0: &[&[i32]] = &[r0, r1];

    let r2: &[i32] = &[];
    let m1: &[&[i32]] = &[r2];

    let t: &[&[&[i32]]] = &[m0, m1];

    assert_d3(&t, 2, &[2, 1], &[&[3, 1], &[0]]);
    assert_d3(&&t, 2, &[2, 1], &[&[3, 1], &[0]]);
}

#[test]
fn mixed_types_d3() {
    // Vec of VecDeque of &[i32]
    let s0: &[i32] = &[1, 2];
    let s1: &[i32] = &[3, 4, 5];
    let s2: &[i32] = &[];

    let vd0 = VecDeque::from([s0, s1]);
    let vd1 = VecDeque::from([s2]);
    let v_vd_s: Vec<VecDeque<&[i32]>> = vec![vd0, vd1];

    assert_d3(&v_vd_s, 2, &[2, 1], &[&[2, 3], &[0]]);

    // VecDeque of &[Vec<i32>]
    let v0 = vec![10, 20, 30];
    let v1 = vec![40];
    let slice_v0: &[Vec<i32>] = &[v0, v1];

    let v2 = vec![50, 60];
    let slice_v1: &[Vec<i32>] = &[v2];

    let vd_s_v: VecDeque<&[Vec<i32>]> = VecDeque::from([slice_v0, slice_v1]);
    assert_d3(&vd_s_v, 2, &[2, 1], &[&[3, 1], &[2]]);

    // Slice of Vec of VecDeque
    let row_vd0 = vec![VecDeque::from([1, 2, 3]), VecDeque::from([4])];
    let row_vd1 = vec![VecDeque::new()];
    let s_v_vd: &[Vec<VecDeque<i32>>] = &[row_vd0, row_vd1];

    assert_d3(&s_v_vd, 2, &[2, 1], &[&[3, 1], &[0]]);
}

#[test]
#[should_panic]
fn vec_d3_out_of_bounds_panics() {
    let tensor = vec![vec![vec![1, 2]]];
    let _ = Card::<D3>::card(&tensor, [0, 5]);
}
