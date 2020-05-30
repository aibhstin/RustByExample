use std::f64;

const S: &'static str = "constant";

fn main() {
    println!("{}", S);

    const N: f64 = 50000000.0;

    const D: f64 = 3e20 / N;
    println!("{}", D);

    println!("{}", D as i64);

    println!("{}", D.sin());
}
