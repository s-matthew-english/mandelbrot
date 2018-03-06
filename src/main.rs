extern crate num;
use num::Complex;

#[allow(dead_code)]
fn complex_square_add_loop(c: Complex<f64>) {
  let mut z = Complex { re: 0.0, im: 0.0};
  loop {
    z = z * z + c;
  }
}

fn main() {
    println!("Hello, world!");
}


use std::str::FromStr;

/// Parse the string 's' as a coordinate pair, like `"400x600"` or `"1.0,0.5"`.
