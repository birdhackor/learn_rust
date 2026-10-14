use std::mem::drop;

struct Tracked {
    id: i32,
}

impl Drop for Tracked {
    fn drop(&mut self) {
        println!("清理 id={}", self.id);
    }
}

fn main() {
    let value = Tracked { id: 1 };
    let view = &value;
    drop(value);
    println!("借用仍要讀={}", view.id);
}
