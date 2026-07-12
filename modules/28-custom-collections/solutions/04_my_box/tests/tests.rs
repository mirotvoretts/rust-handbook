use sol_28_04_my_box::*;

use std::cell::Cell;
use std::rc::Rc;

#[test]
fn deref_read() {
    let b = MyBox::new(5u32);
    assert_eq!(*b, 5);
}

#[test]
fn deref_mut_and_methods() {
    let mut b = MyBox::new(String::from("hi"));
    b.push_str("!"); // метод String через DerefMut
    assert_eq!(&*b, "hi!");
    assert_eq!(b.len(), 3);
}

#[test]
fn owns_heap_value() {
    let v = MyBox::new(vec![1, 2, 3]);
    assert_eq!(v.iter().sum::<i32>(), 6);
}

// Значение с деструктором: MyBox обязан уронить его ровно один раз.
struct Probe(Rc<Cell<u32>>);
impl Drop for Probe {
    fn drop(&mut self) {
        self.0.set(self.0.get() + 1);
    }
}

#[test]
fn drops_inner_exactly_once() {
    let count = Rc::new(Cell::new(0));
    {
        let _b = MyBox::new(Probe(Rc::clone(&count)));
        assert_eq!(count.get(), 0);
    }
    assert_eq!(count.get(), 1);
}
