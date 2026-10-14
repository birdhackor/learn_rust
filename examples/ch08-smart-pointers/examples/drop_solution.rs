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
    let first = Tracked { id: 1 };
    let second = Tracked { id: 2 };
    println!("建立兩份={}、{}", first.id, second.id);
    drop(first);
    println!("第一份已清理，第二份仍是={}", second.id);
    println!("main準備結束");
}
