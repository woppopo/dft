use crate::complex::Complex;

pub mod complex;

pub fn dft(f: impl Fn(usize) -> Complex, n: usize) -> impl Fn(usize) -> Complex {
    use std::f64::consts::PI;

    move |t| {
        (0..n)
            .map(|x| {
                let n = Complex::from(n as f64);
                let theta = -Complex::i() * Complex::from(2.0 * PI * (t * x) as f64) / n;
                f(x) * theta.exp()
            })
            .fold(Complex::zero(), |acc, x| acc + x)
    }
}

pub fn idft(f: impl Fn(usize) -> Complex, n: usize) -> impl Fn(usize) -> Complex {
    use std::f64::consts::PI;

    move |x| {
        let sum = (0..n)
            .map(|t| {
                let n = Complex::from(n as f64);
                let theta = Complex::i() * Complex::from(2.0 * PI * (t * x) as f64) / n;
                f(t) * theta.exp()
            })
            .fold(Complex::zero(), |acc, x| acc + x);

        sum / Complex::from(n as f64)
    }
}

pub fn array_to_fn<T>(array: T) -> impl Fn(usize) -> Complex
where
    T: std::ops::Index<usize, Output = Complex>,
{
    move |i| array[i]
}

pub fn fn_to_array(f: impl Fn(usize) -> Complex, n: usize) -> Vec<Complex> {
    (0..n).map(f).collect()
}

fn main() {
    let f = 1;
    let n = 8;
    let test = |a| Complex::from(std::f64::consts::PI * (a * f) as f64 / (n as f64));

    let before = fn_to_array(test, n);

    let dft = dft(test, n);
    let idft = idft(dft, n);

    let after = fn_to_array(idft, n);

    for (a, b) in before.iter().zip(after.iter()) {
        println!("{} {}", a, b);
    }
}
