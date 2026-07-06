//! 08 (1x) — FizzBuzz. Эталонное решение.

pub fn fizzbuzz(n: u32) -> Vec<String> {
    let mut out = Vec::new();
    for i in 1..=n {
        let s = if i % 15 == 0 {
            String::from("FizzBuzz")
        } else if i % 3 == 0 {
            String::from("Fizz")
        } else if i % 5 == 0 {
            String::from("Buzz")
        } else {
            i.to_string()
        };
        out.push(s);
    }
    out
}
