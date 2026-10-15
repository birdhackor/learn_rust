mod source;

use crate::source::copy_text;
use crate::source::fallback;
use crate::source::first_nonempty;
use crate::source::first_text;
use crate::source::longer;
use crate::source::same;
use crate::source::same_placeholder;

fn main() {
    let mut outside = String::new();
    outside.push_str("Rust tutorial");
    println!("same={}", same(outside.as_str()));
    println!("placeholder={}", same_placeholder(outside.as_str()));

    let saved = {
        let mut inside = String::new();
        inside.push_str("hi");
        println!(
            "兩來源，內層使用={}",
            longer(outside.as_str(), inside.as_str())
        );
        first_text(outside.as_str(), inside.as_str())
    };
    println!("只接第一來源，外層使用={saved}");

    let owned = {
        let mut inside = String::new();
        inside.push_str("Rust");
        copy_text(inside.as_str())
    };
    println!("來源結束後的 owned 結果={}", owned.as_str());
    println!("缺值的替代結果={}", fallback(None, outside.as_str()));

    println!("第一段空，選到={}", first_nonempty("", "Rust"));
    println!("第一段非空，選到={}", first_nonempty("Rust", "Python"));
    println!("兩段皆空，長度={}", first_nonempty("", "").len());
}
