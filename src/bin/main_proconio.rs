use proconio::*;
fn main() {
    input!(n: usize);
    for i in 0..n {
        input!(a: u32, b: f64, c: String);
        assert_eq!(i as u32, a);
        assert_eq!(i as f64 * -1.0e3, b);
        assert_eq!(i.to_string(), c);
    }
}
