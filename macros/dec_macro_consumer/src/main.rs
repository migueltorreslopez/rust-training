extern crate dec_macro_example;

use dec_macro_example::sample;

fn main() {
    let sample_vec = sample![1, 2, 3];
    for v in &sample_vec{
        println!("Vector: {}", v)
    }
}
