use ch04_lifetimes::source::first_text;
use ch04_lifetimes::source::longer;

fn main() {
    let mut outside = String::new();
    outside.push_str("Rust tutorial");
    {
        let mut inside = String::new();
        inside.push_str("hi");
        let selected = longer(outside.as_str(), inside.as_str());
        println!("兩來源契約，內層使用={selected}");
    }

    let only_first = {
        let mut inside = String::new();
        inside.push_str("hi");
        first_text(outside.as_str(), inside.as_str())
    };
    println!("只接第一來源，外層使用={only_first}");
}
