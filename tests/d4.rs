use alloc::collections::VecDeque;
use alloc::vec;
use alloc::vec::Vec;
use orx_col_card::*;

extern crate alloc;

fn assert_d4<C: Card<D4>>(
    col: &C,
    expected_l0: usize,
    expected_l1: &[usize],
    expected_l2: &[&[usize]],
    expected_l3: &[&[&[usize]]],
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

    for (i, matrix) in expected_l3.iter().enumerate() {
        for (j, row) in matrix.iter().enumerate() {
            for (k, &l3) in row.iter().enumerate() {
                assert_eq!(col.card([i, j, k]), l3);
                assert_eq!(col.try_card([i, j, k]), Some(l3));
            }
            assert_eq!(col.try_card([i, j, row.len()]), None);
        }
    }
    assert_eq!(col.try_card([expected_l3.len(), 0, 0]), None);
    assert_eq!(col.try_card([100, 100, 100]), None);
}

#[test]
fn vec_of_vec_of_vec_of_vec_d4() {
    let empty: Vec<Vec<Vec<Vec<i32>>>> = vec![];
    assert_d4(&empty, 0, &[], &[], &[]);

    let d4 = vec![
        vec![
            vec![vec![1, 2, 3], vec![4]],
            vec![vec![]],
        ],
        vec![
            vec![vec![5, 6], vec![7, 8, 9, 10], vec![11]],
        ],
    ];

    assert_d4(
        &d4,
        2,
        &[2, 1],
        &[&[2, 1], &[3]],
        &[
            &[&[3, 1], &[0]],
            &[&[2, 4, 1]],
        ],
    );
    assert_d4(
        &&d4,
        2,
        &[2, 1],
        &[&[2, 1], &[3]],
        &[
            &[&[3, 1], &[0]],
            &[&[2, 4, 1]],
        ],
    );

    // Direct calls with Card::<D4>
    assert_eq!(Card::<D4>::card(&d4, []), 2);
    assert_eq!(Card::<D4>::card(&d4, 0), 2);
    assert_eq!(Card::<D4>::card(&d4, [0]), 2);
    assert_eq!(Card::<D4>::card(&d4, [0, 0]), 2);
    assert_eq!(Card::<D4>::card(&d4, [0, 0, 0]), 3);
    assert_eq!(Card::<D4>::card(&d4, [1, 0, 1]), 4);

    assert_eq!(Card::<D4>::try_card(&d4, []), Some(2));
    assert_eq!(Card::<D4>::try_card(&d4, 1), Some(1));
    assert_eq!(Card::<D4>::try_card(&d4, [1]), Some(1));
    assert_eq!(Card::<D4>::try_card(&d4, [1, 0]), Some(3));
    assert_eq!(Card::<D4>::try_card(&d4, [1, 0, 1]), Some(4));
    assert_eq!(Card::<D4>::try_card(&d4, [1, 0, 3]), None);
    assert_eq!(Card::<D4>::try_card(&d4, [2, 0, 0]), None);
}

#[test]
fn vec_deque_of_vec_deque_of_vec_deque_of_vec_deque_d4() {
    let d4: VecDeque<VecDeque<VecDeque<VecDeque<i32>>>> = VecDeque::from([
        VecDeque::from([
            VecDeque::from([
                VecDeque::from([1, 2, 3]),
                VecDeque::new(),
            ]),
        ]),
    ]);

    assert_d4(
        &d4,
        1,
        &[1],
        &[&[2]],
        &[&[&[3, 0]]],
    );
    assert_d4(
        &&d4,
        1,
        &[1],
        &[&[2]],
        &[&[&[3, 0]]],
    );
}

#[test]
fn slice_of_slice_of_slice_of_slice_d4() {
    let r0: &[i32] = &[1, 2];
    let r1: &[i32] = &[3, 4, 5];
    let m0: &[&[i32]] = &[r0, r1];
    let t0: &[&[&[i32]]] = &[m0];

    let r2: &[i32] = &[];
    let m1: &[&[i32]] = &[r2];
    let t1: &[&[&[i32]]] = &[m1];

    let hyper: &[&[&[&[i32]]]] = &[t0, t1];

    assert_d4(
        &hyper,
        2,
        &[1, 1],
        &[&[2], &[1]],
        &[
            &[&[2, 3]],
            &[&[0]],
        ],
    );
    assert_d4(
        &&hyper,
        2,
        &[1, 1],
        &[&[2], &[1]],
        &[
            &[&[2, 3]],
            &[&[0]],
        ],
    );
}

#[test]
fn mixed_types_d4() {
    // Vec of VecDeque of &[Vec<i32>]
    let v0 = vec![1, 2, 3];
    let v1 = vec![4, 5];
    let slice0: &[Vec<i32>] = &[v0, v1];

    let v2 = vec![6];
    let slice1: &[Vec<i32>] = &[v2];

    let vd0 = VecDeque::from([slice0, slice1]);
    let vd1 = VecDeque::from([slice0]);

    let d4_mixed: Vec<VecDeque<&[Vec<i32>]>> = vec![vd0, vd1];

    assert_d4(
        &d4_mixed,
        2,
        &[2, 1],
        &[&[2, 1], &[2]],
        &[
            &[&[3, 2], &[1]],
            &[&[3, 2]],
        ],
    );
}

#[test]
#[should_panic]
fn vec_d4_out_of_bounds_panics() {
    let hyper = vec![vec![vec![vec![1, 2]]]];
    let _ = Card::<D4>::card(&hyper, [0, 0, 5]);
}


