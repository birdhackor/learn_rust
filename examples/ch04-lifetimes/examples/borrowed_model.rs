use ch04_lifetimes::views::TextView;
use ch04_lifetimes::views::make_view;

fn main() {
    let mut text = String::new();
    text.push_str("Rust tutorial");
    let view = TextView {
        text: text.as_str(),
    };
    println!("欄位借來的文字={}", view.text);

    let built = make_view(text.as_str());
    println!("函式建立的視圖={}，bytes={}", built.text, built.text.len());
}
