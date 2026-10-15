pub mod variance;

use crate::variance::adapt_input;
use crate::variance::adapt_output;

fn byte_len(text: &str) -> usize {
    text.len()
}

fn fixed_label() -> &'static str {
    "Rust"
}

fn main() {
    let accepts_literal: fn(&'static str) -> usize = adapt_input(byte_len);
    println!("給長效literal的callback={}", accepts_literal("Rust"));

    let label = adapt_output(fixed_label);
    println!("保證長效輸出可滿足一般借用={}", label());
    assert_eq!(accepts_literal("Rust"), 4);
    assert_eq!(label(), "Rust");
}
