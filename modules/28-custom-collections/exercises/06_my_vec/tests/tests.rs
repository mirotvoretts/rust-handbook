use ex_28_06_my_vec::*;

use std::cell::Cell;
use std::rc::Rc;

#[test]
fn push_pop_len() {
    let mut v: MyVec<i32> = MyVec::new();
    assert!(v.is_empty());
    for i in 0..10 {
        v.push(i);
    }
    assert_eq!(v.len(), 10);
    assert!(v.capacity() >= 10);
    assert_eq!(v.pop(), Some(9));
    assert_eq!(v.pop(), Some(8));
    assert_eq!(v.len(), 8);
}

#[test]
fn pop_empty() {
    let mut v: MyVec<String> = MyVec::new();
    assert_eq!(v.pop(), None);
}

#[test]
fn indexing_and_get() {
    let mut v: MyVec<u32> = MyVec::new();
    for i in 0..5 {
        v.push(i * i);
    }
    // индексация и get - через Deref в срез
    assert_eq!(v[3], 9);
    assert_eq!(v.get(4), Some(&16));
    assert_eq!(v.get(5), None);
    if let Some(x) = v.get_mut(0) {
        *x = 100;
    }
    assert_eq!(v[0], 100);
}

#[test]
fn deref_slice_ops() {
    let mut v: MyVec<i64> = MyVec::new();
    for i in 1..=100 {
        v.push(i);
    }
    // методы среза доступны через Deref
    assert_eq!(v.iter().sum::<i64>(), 5050);
    assert_eq!(v.first(), Some(&1));
    assert_eq!(v.last(), Some(&100));
    assert_eq!(v.len(), 100);
}

#[test]
fn holds_heap_elements() {
    let mut v: MyVec<String> = MyVec::new();
    v.push(String::from("a"));
    v.push(String::from("bb"));
    v.push(String::from("ccc"));
    assert_eq!(v.iter().map(|s| s.len()).sum::<usize>(), 6);
    assert_eq!(v.pop().as_deref(), Some("ccc"));
}

// Каждый элемент должен быть уронён ровно один раз.
struct Probe(Rc<Cell<u32>>);
impl Drop for Probe {
    fn drop(&mut self) {
        self.0.set(self.0.get() + 1);
    }
}

#[test]
fn drops_all_elements_once() {
    let count = Rc::new(Cell::new(0));
    {
        let mut v: MyVec<Probe> = MyVec::new();
        for _ in 0..50 {
            v.push(Probe(Rc::clone(&count)));
        }
        // pop роняет один элемент (возвращённый Probe умирает в конце строки)
        v.pop();
        assert_eq!(count.get(), 1);
    }
    // остальные 49 роняются в Drop вектора
    assert_eq!(count.get(), 50);
}
