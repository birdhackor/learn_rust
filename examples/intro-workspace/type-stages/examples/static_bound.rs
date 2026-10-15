pub fn require_static<T: 'static>(value: T) -> T {
    value
}

struct Message {
    id: i32,
    text: String,
}

impl Drop for Message {
    fn drop(&mut self) {
        println!("收尾id={}，文字={}", self.id, self.text.as_str());
    }
}

fn main() {
    println!("數字={}", require_static(80));
    println!("literal={}", require_static("Rust"));
    {
        let mut text = String::new();
        text.push_str("Rust");
        let message = require_static(Message { id: 1, text });
        println!("owned文字={}", message.text.as_str());
    }
    println!("block已結束");
}
