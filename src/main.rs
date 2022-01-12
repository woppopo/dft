use crate::complex::Complex;

pub mod complex;

fn main() {
    println!(
        "{}",
        Complex::from(1.0) * Complex::new(0.0, 1.0) * Complex::new(0.0, 1.0)
    );
}
