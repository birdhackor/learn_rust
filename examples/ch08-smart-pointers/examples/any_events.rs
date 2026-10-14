use ch08_smart_pointers::payloads::add_to_number;
use ch08_smart_pointers::payloads::event;
use ch08_smart_pointers::payloads::payload_number;

fn main() {
    let amount: i32 = 80;
    let mut numeric = event(1, amount);
    let updated = add_to_number(&mut numeric, 20);
    println!("事件id={}，數值更新成功={}", numeric.id, updated);
    match payload_number(&numeric) {
        Some(number) => println!("事件中的原數值現在={}", *number),
        None => println!("事件沒有i32資料"),
    }

    let mut text = String::new();
    text.push_str("Rust");
    let mut note = event(2, text);
    println!("文字事件更新數值成功={}", add_to_number(&mut note, 20));
    match note.payload.downcast_ref::<String>() {
        Some(text) => println!("原文字未被改成數字={}", text.as_str()),
        None => println!("事件沒有String資料"),
    }
}
