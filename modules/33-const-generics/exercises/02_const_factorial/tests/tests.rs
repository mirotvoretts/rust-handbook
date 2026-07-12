use ex_33_02_const_factorial::{factorial, fib};

const F5: u64 = factorial(5);
const FIB10: u64 = fib(10);

#[test]
fn const_context() {
    assert_eq!(F5, 120);
    assert_eq!(FIB10, 55);
    let arr = [0u8; factorial(3) as usize];
    assert_eq!(arr.len(), 6);
}

#[test]
fn runtime_values() {
    assert_eq!(factorial(0), 1);
    assert_eq!(fib(0), 0);
    assert_eq!(fib(1), 1);
    assert_eq!(fib(7), 13);
}
