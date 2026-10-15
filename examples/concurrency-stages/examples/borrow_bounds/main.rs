pub mod contracts;

use crate::contracts::require_send;
use crate::contracts::require_send_static;
use crate::contracts::require_sync;

fn main() {
    let mut text = String::new();
    text.push_str("Rust");
    let view = require_sync(text.as_str());
    let view = require_send(view);
    println!("局部文字的reference通過Send，bytes={}", view.len());
    assert_eq!(view.len(), 4);

    {
        let owned = require_send_static(text);
        println!("owned文字通過Send與static，內容={}", owned.as_str());
        assert_eq!(owned.as_str(), "Rust");
    }
    println!("owned值仍隨block結束");
}
