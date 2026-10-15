fn reverse_input<'long: 'short, 'short>(
    operation: fn(&'long str) -> usize,
) -> fn(&'short str) -> usize {
    operation
}

fn main() {}
