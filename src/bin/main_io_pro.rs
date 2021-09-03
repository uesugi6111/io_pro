use io_pro::io_pro::Scanner;
use io_pro::{input, input_inner, read_value};
use std::io;
fn main() {
    let stdin = io::stdin();
    let mut sc = Scanner::new(stdin.lock());
    input!(sc = sc, n: usize);
    for i in 0..n {
        input!(sc = sc, a: u32, b: f64, c: String);
        assert_eq!(i as u32, a);
        assert_eq!(i as f64 * -1.0e3, b);
        assert_eq!(i.to_string(), c);
    }
}
