use alloc::collections::VecDeque;
use alloc::vec;
use alloc::vec::Vec;
use orx_col_card::*;

extern crate alloc;

fn assert_d1<C: Card<D1>>(col: &C, expected_len: usize) {
    assert_eq!(col.card([]), expected_len);
    assert_eq!(col.try_card([]), Some(expected_len));
}

#[test]
fn vec_d1() {
    let vec: Vec<i32> = vec![];
    assert_d1(&vec, 0);

    let mut vec = vec![10, 20, 30];
    assert_d1(&vec, 3);
    assert_d1(&&vec, 3);
    assert_d1(&&mut vec, 3);

    vec.push(40);
    assert_d1(&vec, 4);
}

#[test]
fn vec_deque_d1() {
    let deque: VecDeque<i32> = VecDeque::new();
    assert_d1(&deque, 0);

    let mut deque = VecDeque::from([1, 2, 3, 4, 5]);
    assert_d1(&deque, 5);
    assert_d1(&&deque, 5);
    assert_d1(&&mut deque, 5);

    deque.pop_front();
    assert_d1(&deque, 4);
}

#[test]
fn slice_d1() {
    let empty: &[i32] = &[];
    assert_d1(&empty, 0);

    let array = [1, 2, 3, 4];
    let slice: &[i32] = &array[..];
    assert_d1(&slice, 4);
    assert_d1(&&slice, 4);
}
