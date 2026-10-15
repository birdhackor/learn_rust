fn shorter<'a>(first: &'a str, second: &'a str) -> &'a str {
    if first.len() <= second.len() {
        first
    } else {
        second
    }
}

fn main() {
    let mut outside = String::new();
    outside.push_str("Rust tutorial");
    {
        let mut inside = String::new();
        inside.push_str("hi");
        let first_longer = shorter(outside.as_str(), inside.as_str());
        let second_longer = shorter(inside.as_str(), outside.as_str());
        assert!(first_longer.len() == 2);
        assert!(second_longer.len() == 2);
        println!("第一段較長時選到={first_longer}");
        println!("第二段較長時選到={second_longer}");
    }
    println!("等長選第一段={}", shorter("ab", "CD"));
}
