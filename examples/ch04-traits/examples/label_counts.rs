use ch04_traits::counts::count_labels;
use ch04_traits::counts::owned_text;

fn main() {
    let labels = vec![owned_text("Rust"), owned_text("Python"), owned_text("Rust")];
    let counts = count_labels(labels.as_slice());
    for name in ["Rust", "Python", "JavaScript"] {
        match counts.get(name) {
            Some(count) => println!("{name}={}", *count),
            None => println!("{name}=0（未收錄）"),
        }
    }
    println!("原清單仍有{}個標籤", labels.len());
    assert_eq!(counts.len(), 2);
}
